#pragma once
// KXCapture 自有广告模块：数据类型。素材与规则只来自我们自己的 B 后台，不接任何第三方广告 SDK。
#include <QList>
#include <QString>
#include <QUrl>

namespace kx::ads {

inline const QString kSlotTop = QStringLiteral("top");
inline const QString kSlotBottom = QStringLiteral("bottom");
inline const QString kSlotCarousel = QStringLiteral("carousel");

struct Creative {
    QString id;
    QUrl imageUrl;
    QUrl clickUrl;
    int width = 0;
    int height = 0;
};

struct Placement {
    QString slot;
    bool enabled = false;
    QList<Creative> creatives;
    int intervalMs = 5000; // 仅 carousel 使用
    bool usable() const { return enabled && !creatives.isEmpty(); }
};

} // namespace kx::ads
