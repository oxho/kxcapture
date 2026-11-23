#pragma once
#include "snow_shot/ads/adservice.h"

#include <QWidget>

namespace kx::ads {

// 顶部 / 底部横幅。没有可用素材或广告位关闭时自动隐藏（hide），不占空间。
// 同一素材每次变为可见只记一次展示；点击记一次点击并打开 click_url。
class AdBannerWidget : public QWidget {
    Q_OBJECT
  public:
    AdBannerWidget(AdService* service, const QString& slot, QWidget* parent = nullptr);
    void setMaxBannerHeight(int px);
    int heightForWidth(int w) const override;
    bool hasHeightForWidth() const override { return true; }
    QSize sizeHint() const override;
    const Creative* currentCreative() const { return m_has ? &m_creative : nullptr; }

  protected:
    void paintEvent(QPaintEvent*) override;
    void mousePressEvent(QMouseEvent*) override;
    void showEvent(QShowEvent*) override;
    void resizeEvent(QResizeEvent*) override;

  private:
    void reload();
    void maybeReportImpression();
    AdService* m_service;
    QString m_slot;
    Creative m_creative;
    bool m_has = false;
    bool m_impressionReported = false;
    int m_maxHeight = 120;
};

} // namespace kx::ads
