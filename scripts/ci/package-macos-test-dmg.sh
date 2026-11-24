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

# actions/upload-artifact stores files without POSIX modes. Restore +x on every
# Mach-O file before install copies them into the bundle that gets signed.
chmod_macho_executables() {
    local root="$1"
    local path kind
    [[ -d "$root" ]] || return 0
    while IFS= read -r -d '' path; do
        kind="$(file -b "$path" || true)"
        if [[ "$kind" == *"Mach-O"* ]]; then
            chmod +x "$path"
            printf 'chmod +x %s (%s)\n' "$path" "$(stat -f '%Sp' "$path")"
        fi
    done < <(find "$root" -type f -print0)
}

while IFS= read -r -d '' built_app; do
    chmod_macho_executables "$built_app/Contents/MacOS"
    chmod_macho_executables "$built_app/Contents/Frameworks"
done < <(find "$build" -type d -name '*.app' -print0)

stage_one() {
    local component="$1" bundle="$2" product="$3" volume="$4" filename="$5"
    local prefix stage app
    prefix="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-install.XXXXXX")"
    stage="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-dmg.XXXXXX")"
    cmake --install "$build" --config Release --component "$component" --prefix "$prefix"
    app="$prefix/$bundle"
    [[ -d "$app" ]] || { printf 'Installed bundle is missing: %s\n' "$app" >&2; exit 1; }
    chmod_macho_executables "$app/Contents/MacOS"
    chmod_macho_executables "$app/Contents/Frameworks"
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

assert_mounted_executables() {
    local app="$1"
    local failed=0 path kind mode roots=() main main_path
    [[ -d "$app/Contents/MacOS" ]] && roots+=("$app/Contents/MacOS")
    [[ -d "$app/Contents/Frameworks" ]] && roots+=("$app/Contents/Frameworks")
    main="$(python3 - "$app/Contents/Info.plist" <<'PY'
import plistlib, sys
print(plistlib.load(open(sys.argv[1], 'rb')).get('CFBundleExecutable', ''))
PY
)"
    main_path="$app/Contents/MacOS/$main"
    if [[ ! -f "$main_path" ]]; then
        printf 'FATAL: main executable is missing: %s\n' "$main_path" >&2
        return 1
    fi
    mode="$(stat -f '%Sp' "$main_path")"
    printf 'main executable mode %s %s\n' "$mode" "$main_path"
    if [[ "$mode" != *x* ]]; then
        printf 'FATAL: main executable lacks an executable bit: %s\n' "$main_path" >&2
        failed=1
    fi
    while IFS= read -r -d '' path; do
        kind="$(file -b "$path" || true)"
        if [[ "$kind" != *"Mach-O"* ]]; then
            continue
        fi
        mode="$(stat -f '%Sp' "$path")"
        printf 'mach-o %s %s\n' "$mode" "$path"
        if [[ "$mode" != *x* ]]; then
            printf 'FATAL: Mach-O file lacks an executable bit: %s\n' "$path" >&2
            failed=1
        fi
    done < <(find "${roots[@]}" -type f -print0)
    return "$failed"
}

classify_gatekeeper() {
    local tool="$1" output="$2" rc="$3"
    printf '%s\n' "$output"
    if printf '%s\n' "$output" | grep -Eiq \
        'executable bit|not executable|malformed|sealed resource is missing|invalid Info\.plist|bundle format|code object is not signed at all|main executable.*(missing|absent)|structural'; then
        printf 'FATAL: %s reported an executable-bit or structural failure\n' "$tool" >&2
        return 1
    fi
    if [[ "$rc" -eq 0 ]]; then
        printf 'NON-FATAL: %s accepted the bundle\n' "$tool"
        return 0
    fi
    if printf '%s\n' "$output" | grep -Eiq \
        'notar|not trusted|untrusted|no usable signature|CSSMERR_TP_NOT_TRUSTED|SecTrust|Unnotarized|source=Unnotarized'; then
        printf 'NON-FATAL: %s rejected only because the identity is self-signed or not notarized\n' "$tool"
        return 0
    fi
    printf 'FATAL: %s rejected for a reason other than an untrusted or unnotarized identity\n' "$tool" >&2
    return 1
}

assess_gatekeeper() {
    local app="$1" output rc
    if command -v syspolicy_check >/dev/null 2>&1; then
        printf '\n===== syspolicy_check distribution %s =====\n' "$(basename "$app")"
        set +e
        output="$(syspolicy_check distribution "$app" 2>&1)"
        rc=$?
        set -e
        classify_gatekeeper syspolicy_check "$output" "$rc"
    else
        printf 'syspolicy_check is not available on this runner\n'
    fi
    printf '\n===== spctl --assess --type execute -vv %s =====\n' "$(basename "$app")"
    set +e
    output="$(spctl --assess --type execute -vv "$app" 2>&1)"
    rc=$?
    set -e
    classify_gatekeeper spctl "$output" "$rc"
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
    # codesign --verify does not look at the executable bit. Fail here when a
    # mounted Mach-O file is not executable. Gatekeeper trust failures that
    # come only from this job's self-signed, unnotarized certificate are not
    # fatal; executable-bit and structural failures are.
    assert_mounted_executables "$app"
    printf '\n===== codesign --verify --deep --strict --verbose=2 %s =====\n' "$(basename "$app")"
    codesign --verify --deep --strict --verbose=2 "$app"
    printf '\n===== codesign -dvv %s =====\n' "$(basename "$app")"
    codesign -dvv "$app"
    assess_gatekeeper "$app"
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
