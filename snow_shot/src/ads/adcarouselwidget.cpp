#include "snow_shot/ads/adcarouselwidget.h"

#include <QMouseEvent>
#include <QPainter>

namespace kx::ads {

AdCarouselWidget::AdCarouselWidget(AdService* service, QWidget* parent) : QWidget(parent), m_service(service) {
    setCursor(Qt::PointingHandCursor);
    setToolTip(tr("广告"));
    hide();
    m_timer.setSingleShot(false);
    connect(&m_timer, &QTimer::timeout, this, &AdCarouselWidget::advance);
    connect(service, &AdService::configChanged, this, &AdCarouselWidget::reload);
    connect(service, &AdService::imageReady, this, [this] { update(); });
    reload();
}

void AdCarouselWidget::setMaxBannerHeight(int px) {
    m_maxHeight = qMax(20, px);
    updateGeometry();
}

void AdCarouselWidget::reload() {
    const Placement p = m_service->placement(kSlotCarousel);
    const QString oldId = (m_index < m_placement.creatives.size()) ? m_placement.creatives[m_index].id : QString();
    m_placement = p;
    if (!p.usable()) {
        m_index = 0;
        hide();
        m_timer.stop();
        return;
    }
    // 配置刷新后尽量留在同一张；找不到就回到第一张
    int idx = 0;
    for (int i = 0; i < p.creatives.size(); ++i)
        if (p.creatives[i].id == oldId) idx = i;
    const bool changed = idx != m_index || oldId.isEmpty();
    if (changed) m_reported = false;
    m_index = idx;
    for (const Creative& c : p.creatives) m_service->image(c.imageUrl);
    show();
    updateGeometry();
    updateTimer();
    update();
    if (changed) reportCurrent();
}

void AdCarouselWidget::updateTimer() {
    const bool run = isVisible() && !m_hover && m_placement.usable() && m_placement.creatives.size() > 1;
    if (run) {
        m_timer.setInterval(m_placement.intervalMs);
        if (!m_timer.isActive()) m_timer.start();
    } else {
        m_timer.stop();
    }
}

void AdCarouselWidget::advance() {
    if (!m_placement.usable() || m_placement.creatives.size() < 2) return;
    m_index = (m_index + 1) % m_placement.creatives.size();
    m_reported = false;
    update();
    reportCurrent();
}

void AdCarouselWidget::reportCurrent() {
    // 图片还没下载好时不记；图片到达后由 paintEvent 再补记，保证每次切换只记一次。
    if (m_reported || !m_placement.usable() || !isVisible()) return;
    const Creative& c = m_placement.creatives[m_index];
    if (m_service->image(c.imageUrl).isNull()) return;
    m_reported = true;
    m_service->reportImpression(kSlotCarousel, c);
}

int AdCarouselWidget::heightForWidth(int w) const {
    if (!m_placement.usable()) return 0;
    const Creative& c = m_placement.creatives.first();
    const int cw = c.width > 0 ? c.width : 728;
    const int ch = c.height > 0 ? c.height : 180;
    return qMin(m_maxHeight, qMax(20, w * ch / cw));
}

QSize AdCarouselWidget::sizeHint() const {
    return QSize(728, heightForWidth(728));
}

void AdCarouselWidget::resizeEvent(QResizeEvent* e) {
    QWidget::resizeEvent(e);
    if (m_placement.usable()) setFixedHeight(heightForWidth(width()));
}

void AdCarouselWidget::showEvent(QShowEvent* e) {
    QWidget::showEvent(e);
    updateTimer();
    reportCurrent();
}

void AdCarouselWidget::hideEvent(QHideEvent* e) {
    QWidget::hideEvent(e);
    m_timer.stop();
    m_reported = false; // 再次显示时重新计一次展示
}

void AdCarouselWidget::enterEvent(QEnterEvent*) {
    m_hover = true;
    updateTimer();
}

void AdCarouselWidget::leaveEvent(QEvent*) {
    m_hover = false;
    updateTimer();
}

void AdCarouselWidget::paintEvent(QPaintEvent*) {
    if (!m_placement.usable()) return;
    const Creative& c = m_placement.creatives[m_index];
    const QPixmap pm = m_service->image(c.imageUrl);
    if (pm.isNull()) return;
    reportCurrent();
    QPainter p(this);
    p.setRenderHint(QPainter::SmoothPixmapTransform);
    const QPixmap scaled = pm.scaled(size() * devicePixelRatioF(), Qt::KeepAspectRatio, Qt::SmoothTransformation);
    QRectF target(QPointF(0, 0), QSizeF(scaled.size()) / devicePixelRatioF());
    target.moveCenter(QRectF(rect()).center());
    p.drawPixmap(target.toRect(), scaled);
}

void AdCarouselWidget::mousePressEvent(QMouseEvent* e) {
    if (e->button() == Qt::LeftButton && m_placement.usable())
        m_service->reportClick(kSlotCarousel, m_placement.creatives[m_index]);
}

} // namespace kx::ads
