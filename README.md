# KXCapture

**KXCapture（KX截屏）is a modified version of [Snow Shot](https://github.com/mg-chao/snow-shot) by mg-chao.**
It is free software under the GNU General Public License version 3, or any later version (GPL-3.0-or-later).
Copyright (C) 2025-2026 mg-chao. Modified by oxho, 2026.

This application is not Snow Shot and it is not published by Snow Apps. Homepage, support email, privacy-policy URL, and update feed are not configured. Automatic updates and the upstream account, cloud, and release services are disabled until those values are provided.

English · [简体中文](README.zh-CN.md)

## Features

- Capture a region, the focused window, the current display, a delayed shot, or a scrolling screenshot.
- Annotate with arrows, shapes, text, numbered steps, highlights, and blur. The same tools edit existing images.
- Pin images and clipboard content above other windows.
- Extract text with on-device OCR and read QR codes. Translation, table recognition, and other cloud tools stay off while no API base URL is configured.
- Record the screen with optional system or microphone audio, and export video or GIF, APNG, and WebP animations.
- Stitch images, split them, or sketch on a full-screen canvas.
- Customize shortcuts, toolbars, themes, and export settings. The interface is available in English, Simplified Chinese, and Traditional Chinese.
- Optional MCP integration is off by default.

## Download

No installer, package feed, or automatic update service is configured for KXCapture. Do not use upstream Snow Shot release files, WinGet, Scoop, or Homebrew packages as substitutes for this build.

## Development

Build this repository with the upstream platform guides in `docs/development.md`, `docs-macos-build.md`, and `docs-windows-arm64-build.md`. Those guides still describe the macOS and Windows toolchains. Linux does not configure the application: the project requires Qt 6.12 and has no Linux preset.

Upstream project: <https://github.com/mg-chao/snow-shot>

## License

KXCapture is distributed under GPL-3.0-or-later. The original copyright notice remains Copyright (C) 2025-2026 mg-chao. See [snow_shot/COPYRIGHT](snow_shot/COPYRIGHT) and [LICENSE.md](LICENSE.md). Third-party components keep their own licenses; see [snow_shot/THIRD_PARTY_NOTICES.md](snow_shot/THIRD_PARTY_NOTICES.md).
