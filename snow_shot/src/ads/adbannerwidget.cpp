#include "snow_shot/ads/adbannerwidget.h"

#include <QMouseEvent>
#include <QPainter>

namespace kx::ads {

AdBannerWidget::AdBannerWidget(AdService* service, const QString& slot, QWidget* parent)
    : QWidget(parent), m_service(service), m_slot(slot) {
    setCursor(Qt::PointingHandCursor);
    setToolTip(tr("广告"));
    hide();
    connect(service, &AdService::configChanged, this, &AdBannerWidget::reload);
    connect(service, &AdService::imageReady, this, [this] { update(); });
    reload();
}

void AdBannerWidget::setMaxBannerHeight(int px) {
    m_maxHeight = qMax(20, px);
    updateGeometry();
}

void AdBannerWidget::reload() {
    const Placement p = m_service->placement(m_slot);
    const bool had = m_has;
    const QString oldId = m_creative.id;
    if (p.usable()) {
        m_creative = p.creatives.first();
        m_has = true;
    } else {
        m_has = false;
        m_creative = {};
    }
    if (!had || oldId != m_creative.id) m_impressionReported = false;
    m_has ? (void)m_service->image(m_creative.imageUrl) : void();
    setVisible(m_has);
    updateGeometry();
    update();
    maybeReportImpression();
}

int AdBannerWidget::heightForWidth(int w) const {
    if (!m_has) return 0;
    const int cw = m_creative.width > 0 ? m_creative.width : 728;
    const int ch = m_creative.height > 0 ? m_creative.height : 90;
    return qMin(m_maxHeight, qMax(20, w * ch / cw));
}

QSize AdBannerWidget::sizeHint() const {
    const int w = m_creative.width > 0 ? m_creative.width : 728;
    return QSize(w, heightForWidth(w));
}

void AdBannerWidget::resizeEvent(QResizeEvent* e) {
    QWidget::resizeEvent(e);
    if (m_has) setFixedHeight(heightForWidth(width()));
}

void AdBannerWidget::showEvent(QShowEvent* e) {
    QWidget::showEvent(e);
    maybeReportImpression();
}

void AdBannerWidget::maybeReportImpression() {
    if (!m_has || m_impressionReported || !isVisible()) return;
    const QPixmap pm = m_service->image(m_creative.imageUrl);
    if (pm.isNull()) return; // 图还没到，等 imageReady 后 paint 再记
    m_impressionReported = true;
    m_service->reportImpression(m_slot, m_creative);
}

void AdBannerWidget::paintEvent(QPaintEvent*) {
    if (!m_has) return;
    const QPixmap pm = m_service->image(m_creative.imageUrl);
    if (pm.isNull()) return;
    QPainter p(this);
    p.setRenderHint(QPainter::SmoothPixmapTransform);
    const QPixmap scaled = pm.scaled(size() * devicePixelRatioF(), Qt::KeepAspectRatio, Qt::SmoothTransformation);
    QRectF target(QPointF(0, 0), QSizeF(scaled.size()) / devicePixelRatioF());
    target.moveCenter(QRectF(rect()).center());
    p.drawPixmap(target.toRect(), scaled);
    maybeReportImpression();
}

void AdBannerWidget::mousePressEvent(QMouseEvent* e) {
    if (e->button() == Qt::LeftButton && m_has) m_service->reportClick(m_slot, m_creative);
}

} // namespace kx::ads
