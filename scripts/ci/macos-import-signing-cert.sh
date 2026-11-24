#!/bin/bash
# Import the reused KXCapture signing certificate from GitHub Actions secrets.
# The PKCS#12 and its password are never printed.
set -euo pipefail

keychain="${KXCAPTURE_KEYCHAIN:-${RUNNER_TEMP:-/tmp}/kxcapture-test-signing.keychain-db}"
p12_b64="${KXCAPTURE_MAC_CERT_P12_BASE64:-}"
password="${KXCAPTURE_MAC_CERT_P12_PASSWORD:-}"

if [[ -z "$p12_b64" || -z "$password" ]]; then
    printf 'The long-lived macOS signing certificate is not configured.\n' >&2
    printf 'Set the Actions secrets KXCAPTURE_MAC_CERT_P12_BASE64 and KXCAPTURE_MAC_CERT_P12_PASSWORD.\n' >&2
    printf 'Create them on a Mac with scripts/create-kxcapture-selfsigned-cert.sh.\n' >&2
    exit 1
fi
if [[ -z "${GITHUB_ENV:-}" ]]; then
    printf 'GITHUB_ENV is required so later steps can see the certificate SHA-1.\n' >&2
    exit 2
fi

printf '::add-mask::%s\n' "$password"

if [[ -f "$keychain" ]]; then
    security delete-keychain "$keychain" >/dev/null 2>&1 || rm -f "$keychain"
fi

work="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-import-cert.XXXXXX")"
chmod 700 "$work"
cleanup() {
    rm -rf "$work"
}
trap cleanup EXIT

printf '%s' "$p12_b64" | /usr/bin/openssl base64 -d -A -out "$work/identity.p12"
unset p12_b64

security create-keychain -p "$password" "$keychain"
security set-keychain-settings -lut 28800 "$keychain"
security unlock-keychain -p "$password" "$keychain"
security import "$work/identity.p12" -k "$keychain" -P "$password" -T /usr/bin/codesign -T /usr/bin/security >/dev/null
rm -f "$work/identity.p12"
security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$password" "$keychain" >/dev/null
security list-keychains -d user -s "$keychain" login.keychain-db
security default-keychain -s "$keychain"

sha1="$(security find-certificate -c 'KXCapture Self-Signed' -p "$keychain" | /usr/bin/openssl x509 -noout -fingerprint -sha1)"
identity="$(printf '%s\n' "$sha1" | awk -F= '{print $2}' | tr -d ':' | tr '[:lower:]' '[:upper:]')"
if [[ ! "$identity" =~ ^[A-F0-9]{40}$ ]]; then
    printf 'Refusing to continue without a SHA-1 certificate identity.\n' >&2
    exit 1
fi
{
    printf 'KXCAPTURE_CODESIGN_SHA1=%s\n' "$identity"
    printf 'KXCAPTURE_KEYCHAIN=%s\n' "$keychain"
} >>"$GITHUB_ENV"
printf 'Imported the long-lived KXCapture signing identity into a temporary keychain.\n'
