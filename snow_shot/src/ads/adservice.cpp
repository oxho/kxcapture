#include "snow_shot/ads/adservice.h"

#include <QDesktopServices>
#include <QDir>
#include <QFile>
#include <QJsonArray>
#include <QJsonDocument>
#include <QNetworkAccessManager>
#include <QNetworkReply>
#include <QNetworkRequest>
#include <QSaveFile>
#include <QStandardPaths>
#include <QUrlQuery>

namespace kx::ads {
namespace {
constexpr qint64 kMaxConfigBytes = 1 * 1024 * 1024;
constexpr qint64 kMaxImageBytes = 2 * 1024 * 1024;

bool isHttpUrl(const QUrl& url) {
    const QString s = url.scheme().toLower();
    return url.isValid() && (s == QLatin1String("http") || s == QLatin1String("https")) &&
           !url.host().isEmpty() && url.userName().isEmpty() && url.password().isEmpty();
}
} // namespace

QUrl AdService::defaultBaseUrl() {
    const QByteArray env = qgetenv("KX_AD_BASE_URL");
    if (!env.isEmpty()) {
        const QUrl u = QUrl::fromUserInput(QString::fromUtf8(env));
        if (isHttpUrl(u)) return u;
    }
    return QUrl(QStringLiteral("http://127.0.0.1:8781"));
}

QString AdService::defaultPlatform() {
#if defined(Q_OS_MACOS)
    return QStringLiteral("macos");
#else
    return QStringLiteral("windows");
#endif
}

AdService::AdService(Options options, QObject* parent) : QObject(parent), m_options(std::move(options)) {
    if (!m_options.baseUrl.isValid() || m_options.baseUrl.isEmpty()) m_options.baseUrl = defaultBaseUrl();
    if (m_options.platform.isEmpty()) m_options.platform = defaultPlatform();
    if (m_options.cacheDirectory.isEmpty())
        m_options.cacheDirectory =
            QStandardPaths::writableLocation(QStandardPaths::CacheLocation) + QStringLiteral("/kx-ads");
    m_nam = new QNetworkAccessManager(this);
    m_nam->setTransferTimeout(m_options.requestTimeoutMs);
    m_refreshTimer.setInterval(m_options.refreshIntervalMs);
    connect(&m_refreshTimer, &QTimer::timeout, this, &AdService::refresh);
    loadCounts();
}

AdService::~AdService() = default;

QUrl AdService::configUrl() const {
    QUrl url = m_options.baseUrl;
    url.setPath(QStringLiteral("/v1/config"));
    QUrlQuery q;
    q.addQueryItem(QStringLiteral("app_id"), m_options.appId);
    q.addQueryItem(QStringLiteral("platform"), m_options.platform);
    url.setQuery(q);
    return url;
}

void AdService::start() {
    loadCache();
    refresh();
    m_refreshTimer.start();
}

void AdService::refresh() {
    if (m_configReply) return;
    QNetworkRequest req(configUrl());
    req.setRawHeader("Accept", "application/json");
    req.setAttribute(QNetworkRequest::RedirectPolicyAttribute, QNetworkRequest::NoLessSafeRedirectPolicy);
    m_configReply = m_nam->get(req);
    connect(m_configReply, &QNetworkReply::finished, this, [this] {
        QNetworkReply* r = m_configReply;
        m_configReply = nullptr;
        onConfigReply(r);
    });
}

void AdService::onConfigReply(QNetworkReply* reply) {
    reply->deleteLater();
    const int status = reply->attribute(QNetworkRequest::HttpStatusCodeAttribute).toInt();
    if (status == 404) { // 应用或平台未配置：当作没有广告
        if (!m_placements.isEmpty()) {
            m_placements.clear();
            emit configChanged();
        }
        return;
    }
    if (reply->error() != QNetworkReply::NoError || status != 200) return; // 静默，沿用缓存
    const QByteArray body = reply->read(kMaxConfigBytes + 1);
    if (body.size() > kMaxConfigBytes) return;
    QHash<QString, Placement> parsed;
    if (!parseConfig(body, &parsed)) return;
    m_placements = parsed;
    saveCache(body);
    emit configChanged();
}

bool AdService::parseConfig(const QByteArray& json, QHash<QString, Placement>* out) {
    QJsonParseError err;
    const QJsonDocument doc = QJsonDocument::fromJson(json, &err);
    if (err.error != QJsonParseError::NoError || !doc.isObject()) return false;
    const QJsonObject root = doc.object();
    if (root.value(QStringLiteral("schema_version")).toInt(-1) != 1) return false;
    const QJsonValue pv = root.value(QStringLiteral("placements"));
    if (!pv.isArray()) return false;
    QHash<QString, Placement> result;
    for (const QJsonValue& v : pv.toArray()) {
        const QJsonObject o = v.toObject();
        Placement p;
        p.slot = o.value(QStringLiteral("slot")).toString();
        if (p.slot != kSlotTop && p.slot != kSlotBottom && p.slot != kSlotCarousel) continue;
        p.enabled = o.value(QStringLiteral("enabled")).toBool(false);
        const int iv = o.value(QStringLiteral("interval_ms")).toInt(5000);
        p.intervalMs = qBound(1000, iv, 120000);
        for (const QJsonValue& cv : o.value(QStringLiteral("creatives")).toArray()) {
            const QJsonObject co = cv.toObject();
            Creative c;
            c.id = co.value(QStringLiteral("id")).toString();
            c.imageUrl = QUrl(co.value(QStringLiteral("image_url")).toString());
            c.clickUrl = QUrl(co.value(QStringLiteral("click_url")).toString());
            c.width = co.value(QStringLiteral("width")).toInt(0);
            c.height = co.value(QStringLiteral("height")).toInt(0);
            if (c.id.isEmpty() || !isHttpUrl(c.imageUrl) || !isHttpUrl(c.clickUrl)) continue;
            p.creatives.append(c);
        }
        result.insert(p.slot, p);
    }
    *out = result;
    return true;
}

Placement AdService::placement(const QString& slot) const {
    return m_placements.value(slot);
}

QPixmap AdService::image(const QUrl& url) {
    auto it = m_images.constFind(url);
    if (it != m_images.constEnd()) return it.value();
    if (!isHttpUrl(url) || m_pendingImages.contains(url)) return {};
    m_pendingImages.insert(url, 1);
    QNetworkRequest req(url);
    req.setAttribute(QNetworkRequest::RedirectPolicyAttribute, QNetworkRequest::NoLessSafeRedirectPolicy);
    QNetworkReply* reply = m_nam->get(req);
    connect(reply, &QNetworkReply::finished, this, [this, reply, url] { onImageReply(reply, url); });
    return {};
}

void AdService::onImageReply(QNetworkReply* reply, const QUrl& url) {
    reply->deleteLater();
    m_pendingImages.remove(url);
    if (reply->error() != QNetworkReply::NoError) return;
    const QByteArray data = reply->read(kMaxImageBytes + 1);
    if (data.size() > kMaxImageBytes) return;
    QPixmap pm;
    if (!pm.loadFromData(data)) return;
    m_images.insert(url, pm);
    emit imageReady(url);
}

void AdService::reportImpression(const QString& slot, const Creative& c) {
    QJsonObject s = m_counts.value(slot).toObject();
    s[QStringLiteral("impressions")] = s.value(QStringLiteral("impressions")).toInt() + 1;
    m_counts[slot] = s;
    saveCounts();
    emit impression(slot, c.id);
}

void AdService::reportClick(const QString& slot, const Creative& c) {
    QJsonObject s = m_counts.value(slot).toObject();
    s[QStringLiteral("clicks")] = s.value(QStringLiteral("clicks")).toInt() + 1;
    m_counts[slot] = s;
    saveCounts();
    emit click(slot, c.id);
    if (m_options.openUrlOnClick && isHttpUrl(c.clickUrl)) QDesktopServices::openUrl(c.clickUrl);
}

QJsonObject AdService::localCounts() const { return m_counts; }

void AdService::loadCache() {
    QFile f(m_options.cacheDirectory + QStringLiteral("/config.json"));
    if (!f.open(QIODevice::ReadOnly)) return;
    QHash<QString, Placement> parsed;
    if (parseConfig(f.read(kMaxConfigBytes), &parsed)) {
        m_placements = parsed;
        emit configChanged();
    }
}

void AdService::saveCache(const QByteArray& json) {
    QDir().mkpath(m_options.cacheDirectory);
    QSaveFile f(m_options.cacheDirectory + QStringLiteral("/config.json"));
    if (f.open(QIODevice::WriteOnly)) {
        f.write(json);
        f.commit();
    }
}

void AdService::loadCounts() {
    QFile f(m_options.cacheDirectory + QStringLiteral("/counts.json"));
    if (f.open(QIODevice::ReadOnly)) m_counts = QJsonDocument::fromJson(f.readAll()).object();
}

void AdService::saveCounts() {
    QDir().mkpath(m_options.cacheDirectory);
    QSaveFile f(m_options.cacheDirectory + QStringLiteral("/counts.json"));
    if (f.open(QIODevice::WriteOnly)) {
        f.write(QJsonDocument(m_counts).toJson(QJsonDocument::Compact));
        f.commit();
    }
}

} // namespace kx::ads
