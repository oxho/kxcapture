// 需要本机 B 后台在 KX_AD_BASE_URL（默认 http://127.0.0.1:8781）运行，且已建好 kxcapture 应用。
// 运行: QT_QPA_PLATFORM=offscreen ./kx_ads_tests
#include "snow_shot/ads/adbannerwidget.h"
#include "snow_shot/ads/adcarouselwidget.h"
#include "snow_shot/ads/adservice.h"

#include <QSignalSpy>
#include <QTemporaryDir>
#include <QtTest>

using namespace kx::ads;

static const char* kUnsafeJson = "{\"schema_version\":1,\"placements\":[{\"slot\":\"top\",\"enabled\":true,\"creatives\":[\n          {\"id\":\"a\",\"image_url\":\"http://127.0.0.1/a.png\",\"click_url\":\"javascript:alert(1)\"},\n          {\"id\":\"b\",\"image_url\":\"file:///etc/passwd\",\"click_url\":\"https://example.com\"},\n          {\"id\":\"c\",\"image_url\":\"http://127.0.0.1/c.png\",\"click_url\":\"https://u:p@example.com\"},\n          {\"id\":\"d\",\"image_url\":\"http://127.0.0.1/d.png\",\"click_url\":\"https://example.com/x\",\"width\":728,\"height\":90}]},\n          {\"slot\":\"unknown\",\"enabled\":true,\"creatives\":[]}]}";

class AdsTests : public QObject {
    Q_OBJECT
    QTemporaryDir m_tmp;
    static AdService::Options opts(const QString& dir, const QString& app = QStringLiteral("kxcapture"),
                                   const QString& platform = QStringLiteral("macos")) {
        AdService::Options o;
        o.cacheDirectory = dir;
        o.appId = app;
        o.platform = platform;
        o.openUrlOnClick = false;
        o.requestTimeoutMs = 3000;
        return o;
    }

  private slots:
    void parseRejectsBadInput() {
        QHash<QString, Placement> out;
        QVERIFY(!AdService::parseConfig("not json", &out));
        QVERIFY(!AdService::parseConfig("{\"schema_version\":2,\"placements\":[]}", &out));
        QVERIFY(!AdService::parseConfig("{\"schema_version\":1}", &out));
    }
    void parseDropsUnsafeUrls() {
        QHash<QString, Placement> out;
        const char* json = kUnsafeJson;
        QVERIFY(AdService::parseConfig(json, &out));
        QCOMPARE(out.size(), 1);
        QCOMPARE(out["top"].creatives.size(), 1);
        QCOMPARE(out["top"].creatives[0].id, QString("d"));
    }
    void liveConfigAndWidgets() {
        const QString dir = m_tmp.path() + "/live";
        AdService svc(opts(dir));
        QSignalSpy cfg(&svc, &AdService::configChanged);
        svc.start();
        QTRY_VERIFY_WITH_TIMEOUT(cfg.count() >= 1, 5000);
        QVERIFY(svc.placement("top").usable());
        QCOMPARE(svc.placement("top").creatives.size(), 1);
        QVERIFY(svc.placement("bottom").usable());
        QCOMPARE(svc.placement("carousel").creatives.size(), 2);
        QCOMPARE(svc.placement("carousel").intervalMs, 5000);

        QWidget host;
        host.resize(728, 600);
        AdBannerWidget top(&svc, "top", &host);
        AdBannerWidget bottom(&svc, "bottom", &host);
        AdCarouselWidget car(&svc, &host);
        top.setGeometry(0, 0, 728, 90);
        car.setGeometry(0, 100, 728, 180);
        bottom.setGeometry(0, 300, 728, 90);
        QSignalSpy imp(&svc, &AdService::impression);
        QSignalSpy clk(&svc, &AdService::click);
        host.show();
        // 三个位都应显示并在图片到达后各记一次展示
        QTRY_VERIFY_WITH_TIMEOUT(top.isVisible() && bottom.isVisible() && car.isVisible(), 3000);
        QTRY_VERIFY_WITH_TIMEOUT(imp.count() >= 3, 5000);
        QStringList seenSlots;
        for (auto& a : imp) seenSlots << a[0].toString();
        QVERIFY(seenSlots.contains("top") && seenSlots.contains("bottom") && seenSlots.contains("carousel"));
        QCOMPARE(seenSlots.count("top"), 1);

        const int before = imp.count();
        car.advance();
        QCOMPARE(car.currentIndex(), 1);
        QCOMPARE(imp.count(), before + 1);
        QCOMPARE(imp.last()[0].toString(), QString("carousel"));
        car.advance();
        QCOMPARE(car.currentIndex(), 0);

        QTest::mouseClick(&top, Qt::LeftButton);
        QCOMPARE(clk.count(), 1);
        QCOMPARE(clk.last()[0].toString(), QString("top"));
        QCOMPARE(svc.localCounts()["top"].toObject()["clicks"].toInt(), 1);
        QVERIFY(svc.localCounts()["carousel"].toObject()["impressions"].toInt() >= 3);

        const QString out = qEnvironmentVariable("KX_ADS_SHOT_DIR");
        if (!out.isEmpty()) host.grab().save(out + "/kx-ads-widgets-live.png");
    }
    void unreachableServerKeepsCache() {
        const QString dir = m_tmp.path() + "/live"; // 复用上一个测试写入的缓存
        AdService::Options o = opts(dir);
        o.baseUrl = QUrl("http://127.0.0.1:1");
        AdService svc(o);
        svc.start();
        QVERIFY(svc.placement("top").usable()); // 来自本地缓存，没有网络也有
        QTest::qWait(500);
        QVERIFY(svc.placement("top").usable()); // 失败后仍保留，不清空
    }
    void unknownAppHidesWidgets() {
        AdService svc(opts(m_tmp.path() + "/none", "no-such-app"));
        QWidget host;
        AdBannerWidget top(&svc, "top", &host);
        AdCarouselWidget car(&svc, &host);
        host.show();
        svc.start();
        QTest::qWait(800);
        QVERIFY(!svc.placement("top").usable());
        QVERIFY(!top.isVisible());
        QVERIFY(!car.isVisible());
    }
    void windowsPlatformAlsoConfigured() {
        AdService svc(opts(m_tmp.path() + "/win", "kxcapture", "windows"));
        QSignalSpy cfg(&svc, &AdService::configChanged);
        svc.start();
        QTRY_VERIFY_WITH_TIMEOUT(cfg.count() >= 1, 5000);
        QVERIFY(svc.placement("carousel").usable());
    }
};

QTEST_MAIN(AdsTests)
#include "ads_tests.moc"
