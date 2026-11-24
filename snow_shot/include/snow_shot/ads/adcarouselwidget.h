#pragma once
#include "snow_shot/ads/adservice.h"

#include <QTimer>
#include <QWidget>

namespace kx::ads {

// 轮播位。按 B 后台下发的 interval_ms 自动切换；只有一张素材时不切换；
// 鼠标悬停暂停；不可见时暂停。每次切到一张素材记一次展示。
class AdCarouselWidget : public QWidget {
    Q_OBJECT
  public:
    explicit AdCarouselWidget(AdService* service, QWidget* parent = nullptr);
    void setMaxBannerHeight(int px);
    int heightForWidth(int w) const override;
    bool hasHeightForWidth() const override { return true; }
    QSize sizeHint() const override;
    int currentIndex() const { return m_index; }
    void advance(); // 切到下一张；测试和手动切换用

  protected:
    void paintEvent(QPaintEvent*) override;
    void mousePressEvent(QMouseEvent*) override;
    void enterEvent(QEnterEvent*) override;
    void leaveEvent(QEvent*) override;
    void showEvent(QShowEvent*) override;
    void hideEvent(QHideEvent*) override;
    void resizeEvent(QResizeEvent*) override;

  private:
    void reload();
    void updateTimer();
    void reportCurrent();
    AdService* m_service;
    Placement m_placement;
    int m_index = 0;
    bool m_hover = false;
    bool m_reported = false;
    int m_maxHeight = 180;
    QTimer m_timer;
};

} // namespace kx::ads
