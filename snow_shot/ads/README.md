# KXCapture 自有广告模块（PC：顶部、底部、轮播）

C++/Qt Widgets 客户端模块。素材和规则全部来自我们自己的 B 后台，不接任何第三方广告 SDK。
只统计展示和点击（本地计数，不上传），不收集与广告无关的用户数据。

许可证：随 KXCapture 主体按 **GPL-3.0-or-later** 开源（用户已接受这一义务）。

## 位置

本目录现在在 box 的 `/workspace/kxcapture-ads/`。仓库 `oxho/kxcapture` 建好后，由桌面开发拷入：

- 头文件 → `snow_shot/include/snow_shot/ads/`
- 源码 → `snow_shot/src/ads/`
- 本 `CMakeLists.txt` 可放在 `snow_shot/` 下被 `include()` / `add_subdirectory()`
- 测试 → `snow_shot/tests/` 或保持 `tests/ads_tests.cpp`

库目标：`snow_shot_ads`（静态库，下划线）。测试可执行文件：`snow-shot-ads-tests`（连字符）。
命名空间：`kx::ads`。完整版和 Mini 都链这个库。

主工程必须先走 `cmake/SnowQt.cmake` 的 `snow_find_qt`（Qt 6.12.0）。本文件被主工程引入时**不**调用 `find_package(Qt6)`。单独在本目录自测（box）才用带版本的 `find_package`。

## 和桌面开发对齐的接入位置

广告只进主窗口，不进截图选区、工具条、钉图窗口。

- `top`：contentShell 里、contentHeader 下方、contentArea 上方，主窗口各页都可见
- `bottom`：contentShell 底部、contentArea 下方，同样全局可见
- `carousel`：关于页 `AboutPageWidget`，不进设置/历史

拉失败或 `enabled=false` 时控件隐藏，不影响截图。Win / Mac 同一份代码，只是请求里的 `platform` 不同，由接入方按 `Q_OS_MACOS` / `Q_OS_WIN` 传入（模块在未指定时也会自己判断）。

## 接口

```
GET http://127.0.0.1:8781/v1/config?app_id=kxcapture&platform=macos
GET http://127.0.0.1:8781/v1/config?app_id=kxcapture&platform=windows
```

只读。未知 `app_id` 或平台返回 404；`/v1/*` 的 POST/PUT/DELETE 返回 405。

基地址默认 `http://127.0.0.1:8781`，可被环境变量 `KX_AD_BASE_URL` 覆盖，也可在 `AdService::Options.baseUrl` 里传入。

响应 `schema_version` 必须为 `1`，主要字段：

```json
{
  "schema_version": 1,
  "app_id": "kxcapture",
  "platform": "macos",
  "placements": [
    {"slot": "top", "enabled": true, "creatives": [
      {"id": "c_…", "image_url": "http://127.0.0.1:8781/media/….png",
       "click_url": "https://example.com/…", "width": 728, "height": 90}
    ]},
    {"slot": "bottom", "enabled": true, "creatives": [/* 同上 */]},
    {"slot": "carousel", "enabled": true, "interval_ms": 5000, "creatives": [/* 至少 1 条 */]}
  ]
}
```

客户端会丢掉非 `http`/`https` 的图片或跳转地址，以及带用户名/密码的 URL。非轮播位若下发多条素材，控件只展示第一条（与 B 后台本地测试页一致）。

## 用法摘要

```cpp
kx::ads::AdService::Options opt;
opt.appId = QStringLiteral("kxcapture");
opt.platform = QStringLiteral("macos"); // 或 "windows"
kx::ads::AdService* ads = new kx::ads::AdService(opt, this);
auto* top = new kx::ads::AdBannerWidget(ads, kx::ads::kSlotTop, contentShell);
auto* bottom = new kx::ads::AdBannerWidget(ads, kx::ads::kSlotBottom, contentShell);
auto* carousel = new kx::ads::AdCarouselWidget(ads, aboutPage);
ads->start();
```

## B 后台里的测试配置（不是正式投放）

已在本机 B 后台（`127.0.0.1:8781`）建好：

- 应用 id：`kxcapture`，名称「KXCapture（KX截屏）」，平台 macos + windows，总开关开
- 广告位：macos / windows 的 top、bottom、carousel 均启用；轮播间隔 5000 ms
- 素材（名称均带「测试数据」）：
  - `c_5d8bf2796c65` 顶部（728×90）
  - `c_40b1b3655380` 底部（728×90）
  - `c_520c0e7d1b18` 轮播 1（728×180）
  - `c_bcd4958b09fd` 轮播 2（728×180）
- 管理页：http://127.0.0.1:8781/admin/apps/kxcapture
- 测试页：http://127.0.0.1:8781/test?app_id=kxcapture&platform=macos
- 重复写入脚本：`tools/setup_kxcapture_in_b_admin.py`（只打本机 `127.0.0.1`）

这些是测试数据，正式投放前不要当成生产配置。当前只写在 **box** 上的 B 后台；Mac 上 `/Users/zjl/erkai-b-admin` 是否已有同一份配置见交付说明（未同步前是未验证）。

## 在本目录自测（box）

需要本机 `127.0.0.1:8781` 已启动且含上述配置。

```bash
cmake -S . -B build -G Ninja -DSNOW_SHOT_ADS_BUILD_TESTS=ON
cmake --build build
QT_QPA_PLATFORM=offscreen ./build/snow-shot-ads-tests
```
