#!/bin/bash
# Stage the release bundles and wrap them in ULMO disk images.
# scripts/package-snow-shot.sh is not used: it requires the publisher private key
# (snow-shot-macos-signing.py identity) and the Sparkle Ed25519 key (check-key,
# prepare-release). This test path signs with KXCAPTURE_CODESIGN_SHA1 only.
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
preset="${1:-snow-shot-macos-arm64-release}"
build="$root/build/$preset"
out="${2:-$root/artifacts/macos}"
identity="${KXCAPTURE_CODESIGN_SHA1:?Set KXCAPTURE_CODESIGN_SHA1 to the job-local certificate SHA-1}"
version="$(sed -n 's/^set(SNOW_SHOT_VERSION "\([^"]*\)").*/\1/p' "$root/CMakeLists.txt")"
[[ -n "$version" ]] || { printf 'Could not read SNOW_SHOT_VERSION.\n' >&2; exit 1; }
case "$preset" in
    *-arm64-*) arch=arm64 ;;
    *-x64-*) arch=x86_64 ;;
    *) printf 'Unsupported preset: %s\n' "$preset" >&2; exit 1 ;;
esac

mkdir -p "$out"
rm -f "$out"/*.dmg "$out/SHA256SUMS.txt"

stage_one() {
    local component="$1" bundle="$2" product="$3" volume="$4" filename="$5"
    local prefix stage app
    prefix="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-install.XXXXXX")"
    stage="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-dmg.XXXXXX")"
    cmake --install "$build" --config Release --component "$component" --prefix "$prefix"
    app="$prefix/$bundle"
    [[ -d "$app" ]] || { printf 'Installed bundle is missing: %s\n' "$app" >&2; exit 1; }
    ditto "$app" "$stage/$product"
    ln -s /Applications "$stage/Applications"
    local image="$out/$filename"
    hdiutil create -volname "$volume" -srcfolder "$stage" -ov -format ULMO "$image"
    # The repository signs the disk image in cmake/SignSnowShotMacOSDmg.cmake.
    # That hook uses an ad-hoc signature because the image is a container; the
    # application inside keeps the project identity. Match that split here, but
    # sign the container with this job's certificate instead of ad-hoc '-'.
    codesign --force --sign "$identity" "$image"
    codesign --verify --verbose=2 "$image"
    rm -rf "$prefix" "$stage"
    LAST_IMAGE="$image"
}

verify_image() {
    local image="$1"
    local mount app
    mount="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-mount.XXXXXX")"
    hdiutil attach -readonly -nobrowse -noautoopen -mountpoint "$mount" "$image"
    app=""
    for candidate in "$mount"/*.app; do
        if [[ -d "$candidate" ]]; then
            app="$candidate"
            break
        fi
    done
    if [[ -z "$app" ]]; then
        printf 'DMG has no application: %s\n' "$image" >&2
        hdiutil detach "$mount" || true
        exit 1
    fi
    printf '\n===== codesign --verify --deep --strict --verbose=2 %s =====\n' "$(basename "$app")"
    codesign --verify --deep --strict --verbose=2 "$app"
    printf '\n===== codesign -dvv %s =====\n' "$(basename "$app")"
    codesign -dvv "$app"
    python3 - "$app/Contents/Info.plist" <<'PY'
import plistlib
import sys
info = plistlib.load(open(sys.argv[1], 'rb'))
print('CFBundleIdentifier=' + str(info.get('CFBundleIdentifier')))
print('CFBundleName=' + str(info.get('CFBundleName')))
print('CFBundleDisplayName=' + str(info.get('CFBundleDisplayName')))
print('CFBundleExecutable=' + str(info.get('CFBundleExecutable')))
PY
    hdiutil detach "$mount"
    rmdir "$mount" 2>/dev/null || true
}

stage_one SnowShot kxcapture.app 'KXCapture.app' 'KXCapture' "KXCapture-${version}-macos-${arch}.dmg"
verify_image "$LAST_IMAGE"
if grep -Eq '^SNOW_APPS_BUILD_SNOW_SHOT_MINI:BOOL=(ON|TRUE|1)$' "$build/CMakeCache.txt"; then
    stage_one SnowShotMini kxcapture_mini.app 'KXCapture Mini.app' 'KXCapture Mini' "KXCapture-Mini-${version}-macos-${arch}.dmg"
    verify_image "$LAST_IMAGE"
fi

(
    cd "$out"
    shasum -a 256 ./*.dmg | tee SHA256SUMS.txt
    stat -f 'size_bytes %z %N' ./*.dmg
)
