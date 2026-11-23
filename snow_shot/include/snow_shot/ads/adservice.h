#pragma once
#include "snow_shot/ads/adtypes.h"

#include <QHash>
#include <QJsonObject>
#include <QObject>
#include <QPixmap>
#include <QTimer>

class QNetworkAccessManager;
class QNetworkReply;

namespace kx::ads {

// 拉取 B 后台 GET /v1/config，缓存到本地，提供广告位数据和图片。
// 只统计展示和点击（本地计数 + 信号），不收集、不上传任何与广告无关的用户数据；
// 请求里只带 app_id 和 platform 两个查询参数。
// 所有失败都静默：网络不通、JSON 非法、404 都不会弹窗，也不阻塞主界面。
class AdService final : public QObject {
    Q_OBJECT
  public:
    struct Options {
        // 默认本地 B 后台；可被环境变量 KX_AD_BASE_URL 覆盖（见 defaultBaseUrl）。
        QUrl baseUrl;
        QString appId = QStringLiteral("kxcapture");
        QString platform; // 为空时按编译平台取 "macos" 或 "windows"
        QString cacheDirectory; // 为空时用 QStandardPaths::CacheLocation/kx-ads
        int refreshIntervalMs = 15 * 60 * 1000;
        int requestTimeoutMs = 8000;
        bool openUrlOnClick = true; // 测试里可关掉
    };

    explicit AdService(Options options, QObject* parent = nullptr);
    explicit AdService(QObject* parent = nullptr) : AdService(Options{}, parent) {}
    ~AdService() override;

    static QUrl defaultBaseUrl();
    static QString defaultPlatform();

    // 读本地缓存（若有），随后拉一次远端，并按 refreshIntervalMs 定时刷新。
    void start();
    void refresh();

    Placement placement(const QString& slot) const;
    // 拿图片；未缓存时发起下载，完成后发 imageReady。返回的 pixmap 可能为空。
    QPixmap image(const QUrl& url);

    // 控件调用：记录一次展示 / 点击（点击还会用系统浏览器打开 click_url，仅 http/https）。
    void reportImpression(const QString& slot, const Creative& creative);
    void reportClick(const QString& slot, const Creative& creative);

    // 本地计数 {slot: {impressions, clicks}}，只存在本机，不上传。
    QJsonObject localCounts() const;

    // 供测试和解析复用：JSON 文本 -> placements；非法返回 false。
    static bool parseConfig(const QByteArray& json, QHash<QString, Placement>* out);

  signals:
    void configChanged();
    void imageReady(const QUrl& url);
    void impression(const QString& slot, const QString& creativeId);
    void click(const QString& slot, const QString& creativeId);

  private:
    void onConfigReply(QNetworkReply* reply);
    void onImageReply(QNetworkReply* reply, const QUrl& url);
    void loadCache();
    void saveCache(const QByteArray& json);
    void loadCounts();
    void saveCounts();
    QUrl configUrl() const;

    Options m_options;
    QNetworkAccessManager* m_nam = nullptr;
    QHash<QString, Placement> m_placements;
    QHash<QUrl, QPixmap> m_images;
    QHash<QUrl, int> m_pendingImages;
    QJsonObject m_counts;
    QTimer m_refreshTimer;
    QNetworkReply* m_configReply = nullptr;
};

} // namespace kx::ads
