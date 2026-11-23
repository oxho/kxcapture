# KX截屏

**KX截屏（KXCapture）是 [Snow Shot](https://github.com/mg-chao/snow-shot)（作者 mg-chao）的修改版本。**
本软件是自由软件，采用 GNU 通用公共许可证第 3 版或更新版本（GPL-3.0-or-later）。
Copyright (C) 2025-2026 mg-chao. Modified by oxho, 2026.

本应用不是 Snow Shot，也不是 Snow Apps 发布的官方版本。主页、支持邮箱、隐私政策地址和更新源都尚未配置。在提供这些地址之前，自动更新以及上游账号、云服务和发布通道保持关闭。

[English](README.md) · 简体中文

## 特性

- 区域截图、焦点窗口、当前显示器、延时截图和滚动长截图。
- 箭头、形状、文字、序号、高亮、模糊等标注，也可编辑已有图片。
- 将图片和剪贴板内容固定在其他窗口之上。
- 使用本机 OCR 提取文字并识别二维码。未配置 API 基址时，翻译、表格识别和其他云功能保持关闭。
- 录制屏幕，可加入系统音频或麦克风，并导出视频或 GIF、APNG、WebP 动图。
- 拼接、分割图片，或在全屏画布上涂写。
- 自定义快捷键、工具栏、主题和导出选项。界面提供 English、简体中文和繁體中文。
- MCP 集成默认关闭。

## 下载

KX截屏尚未配置安装包、软件源或自动更新服务。请不要把上游 Snow Shot 的发布文件、WinGet、Scoop 或 Homebrew 包当作本版本的替代安装方式。

Windows 更新公钥 id 是 `kxcapture-2026-11`，macOS Sparkle 账户是 `kxcapture-sparkle`。正式配置里的更新地址保持为空。本机调试源（不是正式地址）是 `http://127.0.0.1:8782/sparkle/` 和 `http://127.0.0.1:8782/github/repos/oxho/kxcapture/releases`。正式包和商店包的 `allowLocalHttp` 保持关闭，因此除非调试路径单独打开这个开关，这些 http 地址会被拒绝。

## 开发

在本仓库中构建。平台工具链说明见 `docs/development.md`、`docs-macos-build.md` 和 `docs-windows-arm64-build.md`。Linux 上不能配置该应用：工程要求 Qt 6.12，且没有 Linux 预设。

上游项目：<https://github.com/mg-chao/snow-shot>

## 许可证

KX截屏以 GPL-3.0-or-later 分发。原始版权声明 Copyright (C) 2025-2026 mg-chao 予以保留。见 [snow_shot/COPYRIGHT](snow_shot/COPYRIGHT) 和 [LICENSE.md](LICENSE.md)。第三方组件仍适用各自的许可证，见 [snow_shot/THIRD_PARTY_NOTICES.md](snow_shot/THIRD_PARTY_NOTICES.md)。
