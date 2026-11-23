# KXCapture brand checklist status

Checked against `uploads/brand-checklist.md` items 1–78 on branch `cursor/rebrand-kxcapture-7039`. The SHA is the commit that set the current value, not this status file.

| # | File | Status | Commit | Reason if not fully done |
|---|---|---|---|---|
| 1 | `CMakeLists.txt` | partly | `53fa74ea88c6` | `SNOW_SHOT_PRODUCT_NAME` is English `KXCapture`. Chinese `KX截屏` is the `EditionMetadata` translation and the zh InfoPlist/NSIS strings, not this CMake literal. |
| 2 | `CMakeLists.txt` | done | `53fa74ea88c6` | — |
| 3 | `CMakeLists.txt` | done | `53fa74ea88c6` | — |
| 4 | `CMakeLists.txt` | done | `53fa74ea88c6` | — |
| 5 | `snow_shot/include/snow_shot/app/edition.h` | done | `53fa74ea88c6` | — |
| 6 | `snow_shot/include/snow_shot/app/edition.h` | done | `53fa74ea88c6` | — |
| 7 | `snow_shot/include/snow_shot/app/edition.h` | done | `53fa74ea88c6` | — |
| 8 | `snow_shot/include/snow_shot/app/edition.h` | done | `53fa74ea88c6` | — |
| 9 | `snow_shot/include/snow_shot/app/edition.h` | done | `53fa74ea88c6` | — |
| 10 | `snow_shot/src/app/main.cpp` | done | `53fa74ea88c6` | — |
| 11 | `snow_image_viewer/src/app/application_setup.cpp` | partly | `53fa74ea88c6` | Organization is `oxho` and the display name is `KX截屏 看图` / `KXCapture Image Viewer`. The organization domain is empty because no domain was supplied. |
| 12 | `snow_shot/packaging/macos/Info.plist.in` | done | `53fa74ea88c6` | — |
| 13 | `snow_shot/packaging/macos/Info-mini.plist.in` | done | `53fa74ea88c6` | — |
| 14 | `snow_shot/packaging/macos/{en,zh-Hans,zh-Hant}.lproj/InfoPlist.strings` and `mini/*.lproj` | done | `53fa74ea88c6` | — |
| 15 | InfoPlist usage strings and `Info.plist.in` | done | `53fa74ea88c6` | — |
| 16 | `snow_shot/packaging/InstallerStrings.nsh` | partly | `53fa74ea88c6` | `SNOW_SHOT_INSTALLER_PRODUCT_NAME` is `KXCapture`. The zh_CN and zh_TW installer strings use `KX截屏`. |
| 17 | `snow_shot/packaging/i18n/{en_US,zh_CN,zh_TW}.nsh` | done | `53fa74ea88c6` | — |
| 18 | `cmake/SnowShotMiniPackage.cmake` | done | `53fa74ea88c6` | — |
| 19 | `CMakeLists.txt` | done | `53fa74ea88c6` | — |
| 20 | `snow_shot/i18n/**/*.ts` | done | `53fa74ea88c6` | — |
| 21 | `snow_shot/src/presentation/components/aboutpagewidget.cpp` | partly | `53fa74ea88c6` | The title follows the translated product name and the mg-chao credit stays. Website, project, QQ, privacy, and terms links stay hidden because those URLs were not supplied. |
| 22 | `README.md`, `README.zh-CN.md`, `snow_shot/packaging/README.txt`, `README-mini.txt` | done | `53fa74ea88c6` | — |
| 23 | `scripts/package-snow-shot.ps1`, `scripts/install-snow-shot-macos.sh`, `homebrew/`, Scoop and WinGet scripts | partly | `4c3ccf5fbdb6` | Visible names and the WinGet id are KXCapture / `oxho.kxcapture`. Release asset names, Scoop ids, and `scripts/snow-shot-editions.ps1` registry names still use `snow-shot` / `SnowShot`. `docs-macos-build.md` and `docs/snow-shot-releases.md` still describe the upstream tap and `mg-chao.snow-shot`. |
| 24 | `CMakeLists.txt` | done | `53fa74ea88c6` | — |
| 25 | `cmake/SnowShotMacOS.cmake` | done | `53fa74ea88c6` | — |
| 26 | `scripts/snow-shot-macos-signing.py` | done | `53fa74ea88c6` | — |
| 27 | `cmake/SnowShotMini.cmake` | done | `53fa74ea88c6` | — |
| 28 | `cmake/SnowShotMiniMacOS.cmake` | done | `53fa74ea88c6` | — |
| 29 | `snow_shot/packaging/macos/Info.plist.in` | done | `53fa74ea88c6` | — |
| 30 | `snow_shot/packaging/macos/Info-mini.plist.in` | done | `53fa74ea88c6` | — |
| 31 | `snow_shot/packaging/msix/store-identity.json` | partly | `53fa74ea88c6` | Upstream Name, Publisher, package family, and StoreId were removed. Partner Center values were not supplied, so every field is an empty string. |
| 32 | `snow_shot/packaging/msix/AppxManifest.xml.in` | done | `53fa74ea88c6` | — |
| 33 | `snow_shot/packaging/msix/AppxManifest.xml.in` | done | `53fa74ea88c6` | — |
| 34 | Windows AUMID | done | `53fa74ea88c6` | — |
| 35 | `snow_shot/src/app/mcp/*`, `snow_shot/rust/snow-shot-mcp/src/discovery.rs` | done | `53fa74ea88c6` | — |
| 36 | `cmake/SnowShotMacOSPackage.cmake`, `cmake/SnowShotMiniPackage.cmake` | done | `53fa74ea88c6` | — |
| 37 | `snow_shot/resources/app-icon.svg` | done | `53fa74ea88c6` | — |
| 38 | `snow_shot/resources/app-icon.ico` | done | `53fa74ea88c6` | — |
| 39 | `snow_shot/resources/windows-app-icon.rc` | done | `53fa74ea88c6` | — |
| 40 | `snow_shot/resources/windows-app-resource.rc.in` | done | `53fa74ea88c6` | — |
| 41 | `snow_shot/src/platform/windows/explorercommand/explorercommand.rc.in` | done | `53fa74ea88c6` | — |
| 42 | `cmake/SnowShotMacOS.cmake` | done | `53fa74ea88c6` | — |
| 43 | `snow_shot/resources/app-icon-titlebar.svg` | partly | `53fa74ea88c6` | The old green and purple mark is gone. The titlebar file is the full teal app icon, not a separate simplified titlebar drawing. |
| 44 | `snow_shot/resources/snow-shot-tray-*.png` | partly | `53fa74ea88c6` | Tray artwork was replaced in the existing filenames. The running app still loads those files, not the separate 16/24/32 taskbar set stored under `snow_shot/resources/brand/`. |
| 45 | `snow_shot/resources/about/illustration.svg` | done | `53fa74ea88c6` | — |
| 46 | `snow_shot/include/snow_shot/presentation/components/icons/snowshoticons.h` | partly | `53fa74ea88c6` | The icon pack was regenerated from the new artwork. The header filename and snowshot symbol names were left in place. |
| 47 | `snow_shot/packaging/msix/Assets/` | done | `53fa74ea88c6` | — |
| 48 | `snow_shot/packaging/macos/dmg-background.svg` and the mini DMG assets | done | `53fa74ea88c6` | — |
| 49 | `snow_shot/resources/app-icon.svg` | done | `53fa74ea88c6` | — |
| 50 | `snow_shot/resources/app-icon-titlebar.svg` | done | `53fa74ea88c6` | — |
| 51 | Theme tokens and remaining brand colors | partly | `53fa74ea88c6` | Product resources no longer use `#81D550` or `#9254DE`. `ant_design_qt/tests/icon_core_tests.cpp` still contains `#9254DE` as a library test sample. |
| 52 | `cmake/SnowShotSparkle.cmake` | partly | `53fa74ea88c6` | The upstream GitHub feed base is gone. No replacement Sparkle base was supplied, so the committed value is empty and the appcast URL is empty. |
| 53 | `cmake/SnowShotSparkle.cmake` | partly | `53fa74ea88c6` | The upstream Gitee feed base is gone. No replacement mirror was supplied, so the committed value is empty. |
| 54 | `snow_shot/packaging/macos/Info.plist.in`, `Info-mini.plist.in` | partly | `67fb63497b59` | `SUPublicEDKey` is the supplied Sparkle key. `SUFeedURL` stays empty because no feed base was supplied. |
| 55 | `snow_shot/packaging/macos/sparkle.json` | done | `67fb63497b59` | — |
| 56 | `snow_shot/include/snow_shot/update/updateservice.h`, `snow_shot/rust/snow-shot-updater/src/github.rs` | partly | `53fa74ea88c6` | The mg-chao releases URL is gone. The Windows GitHub endpoint is an empty `QUrl`, and the updater reads `KXCAPTURE_UPDATE_REPOSITORY` with an empty default. |
| 57 | `updateservice.h`, `snow_shot/rust/snow-shot-updater/src/service.rs` | partly | `53fa74ea88c6` | The mg-chao Gitee releases URL is gone. The Gitee endpoint is an empty `QUrl` unless a repository env var is set. |
| 58 | Updater and packaging download bases | partly | `4c3ccf5fbdb6` | The in-app updater has no download base. Scoop, WinGet, Homebrew, and the macOS installer name `oxho/kxcapture` so their fixture tests parse. That is this repository, not a separate production update host. |
| 59 | `snow_shot/resources/update-trusted-keys.json` | done | `67fb63497b59` | — |
| 60 | `snow_shot/packaging/macos/release-signing.pem` | partly | `53fa74ea88c6` | The upstream Snow Shot Project Release certificate was removed. No replacement certificate was supplied, so the file is a comment and is not a loadable certificate. |
| 61 | `scripts/install-snow-shot-macos.sh`, `homebrew/` | partly | `4c3ccf5fbdb6` | The installer and cask generator use `oxho/kxcapture` instead of mg-chao. `docs-macos-build.md` still documents `mg-chao/tap/snow-shot`. |
| 62 | `CMakeLists.txt` | partly | `53fa74ea88c6` | `SNOW_SHOT_HOMEPAGE_URL` is empty. No homepage was supplied. |
| 63 | `CMakeLists.txt` | partly | `53fa74ea88c6` | `SNOW_SHOT_PRIVACY_POLICY_URL` is empty. No privacy URL was supplied. |
| 64 | `CMakeLists.txt` | partly | `53fa74ea88c6` | `SNOW_SHOT_SUPPORT_EMAIL` is empty. No support email was supplied. |
| 65 | `snow_shot/CMakeLists.txt` | partly | `53fa74ea88c6` | Website, project, and QQ URLs are empty, so those about-page links stay hidden. |
| 66 | `snow_shot/CMakeLists.txt`, `cmake/SnowWorkspace.cmake`, `CMakePresets.json` | partly | `53fa74ea88c6` | Release API bases are empty, so account and cloud calls do not leave the machine. No replacement API origin was supplied. |
| 67 | `CMakePresets.json` | partly | `53fa74ea88c6` | `test.snowshot.top` is gone. The Windows debug preset still sets `SNOW_APPS_API_BASE_URL` to `http://localhost:5101`. |
| 68 | `snow_shot/src/network/snowshotapiclient.cpp` | partly | `53fa74ea88c6` | Request paths are unchanged. The base URL is empty unless `KXCAPTURE_API_BASE_URL` is set, so the client does not call `snowshot.top`. |
| 69 | `snowshotapiclient.cpp`, `snow_shot/src/app/accountsessionservice.cpp` | partly | `53fa74ea88c6` | The env var is `KXCAPTURE_API_BASE_URL`. Account calls return unconfigured when the origin is empty. No replacement account service was supplied. |
| 70 | `snow_shot/src/presentation/components/aboutpagewidget.cpp` | partly | `53fa74ea88c6` | The QQ group URL is empty and the entry stays hidden. No replacement support channel was supplied. |
| 71 | Website, terms, and installer download links | partly | `53fa74ea88c6` | `snowshot.top` homepage, privacy, terms, and `setup/latest.exe` links are empty. No replacement site was supplied. |
| 72 | `scripts/package-snow-shot.ps1`, `snow_shot/packaging/snow-shot-ocr-asset-manifest*.json` | partly | `53fa74ea88c6` | The ModelScope download URL is empty and packaging throws unless `KXCAPTURE_OCR_MODEL_BASE_URL` or an archive is passed. Archive names still use `snow-ocr-runtime`, and OCR requests still send a ModelScope Referer. |
| 73 | `CMakeLists.txt` | done | `53fa74ea88c6` | — |
| 74 | `snow_shot/src/presentation/components/aboutpagewidget.cpp` | done | `53fa74ea88c6` | — |
| 75 | `snow_shot/packaging/macos/Info.plist.in` | done | `53fa74ea88c6` | — |
| 76 | `snow_shot/packaging/README.txt`, `README-mini.txt` | partly | `53fa74ea88c6` | The Support line is blank. No support email was supplied. |
| 77 | Repository-wide user-visible Snow Shot / Snow Apps strings | partly | `4c3ccf5fbdb6` | App translations no longer present the product as Snow Shot. Internal identifiers remain, and `docs-macos-build.md`, `docs/snow-shot-releases.md`, and `scripts/close-legacy-issues.ps1` still name the upstream project. |
| 78 | CMake and script filenames | done | `53fa74ea88c6` | — |
