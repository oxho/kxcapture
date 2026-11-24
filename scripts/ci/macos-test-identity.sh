#!/bin/bash
# Job-local KXCapture test signing identity. The private key stays in a temporary
# keychain and is deleted before the job uploads artifacts. Nothing here reads or
# writes the publisher certificate in snow_shot/packaging/macos/release-signing.pem.
set -euo pipefail

action="${1:-}"
keychain="${KXCAPTURE_KEYCHAIN:-${RUNNER_TEMP:-/tmp}/kxcapture-test-signing.keychain-db}"

delete_keychain() {
    if [[ -f "$keychain" ]]; then
        security delete-keychain "$keychain" >/dev/null 2>&1 || rm -f "$keychain"
    fi
    security list-keychains -d user -s login.keychain-db >/dev/null 2>&1 || true
}

if [[ "$action" == delete ]]; then
    delete_keychain
    printf 'Deleted temporary signing keychain.\n'
    exit 0
fi
if [[ "$action" != create ]]; then
    printf 'Usage: %s create|delete\n' "$0" >&2
    exit 2
fi
if [[ -z "${GITHUB_ENV:-}" ]]; then
    printf 'GITHUB_ENV is required so later steps can see the certificate SHA-1.\n' >&2
    exit 2
fi

delete_keychain
work="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-test-identity.XXXXXX")"
chmod 700 "$work"
cleanup() {
    rm -rf "$work"
}
trap cleanup EXIT

# Apple's security(1) rejects PKCS#12 files written by Homebrew OpenSSL 3.
# The system LibreSSL openssl produces an importable identity.
openssl=/usr/bin/openssl
password="$("$openssl" rand -hex 32)"
# Mask the keychain password if the runner echoes a failing command line.
printf '::add-mask::%s\n' "$password"

config="$work/certificate.cnf"
cat >"$config" <<'EOF'
[req]
prompt = no
distinguished_name = subject
x509_extensions = signing
[subject]
CN = KXCapture Self-Signed
[signing]
basicConstraints = critical,CA:true
keyUsage = critical,digitalSignature,keyCertSign
extendedKeyUsage = codeSigning
EOF

"$openssl" req -new -newkey rsa:3072 -nodes -x509 -days 2 \
    -config "$config" \
    -keyout "$work/private.pem" \
    -out "$work/certificate.pem" >/dev/null
"$openssl" pkcs12 -export \
    -inkey "$work/private.pem" \
    -in "$work/certificate.pem" \
    -out "$work/identity.p12" \
    -passout "pass:${password}" >/dev/null
rm -f "$work/private.pem"

security create-keychain -p "$password" "$keychain"
security set-keychain-settings -lut 28800 "$keychain"
security unlock-keychain -p "$password" "$keychain"
security import "$work/identity.p12" -k "$keychain" -P "$password" -T /usr/bin/codesign -T /usr/bin/security >/dev/null
rm -f "$work/identity.p12"
security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$password" "$keychain" >/dev/null
security list-keychains -d user -s "$keychain" login.keychain-db
security default-keychain -s "$keychain"

subject="$("$openssl" x509 -in "$work/certificate.pem" -noout -subject)"
sha1="$("$openssl" x509 -in "$work/certificate.pem" -noout -fingerprint -sha1)"
sha256="$("$openssl" x509 -in "$work/certificate.pem" -noout -fingerprint -sha256)"
printf 'Test signing certificate subject: %s\n' "$subject"
printf 'Test signing certificate %s\n' "$sha1"
printf 'Test signing certificate %s\n' "$sha256"
identity="$(printf '%s\n' "$sha1" | awk -F= '{print $2}' | tr -d ':' | tr '[:lower:]' '[:upper:]')"
if [[ ! "$identity" =~ ^[A-F0-9]{40}$ ]]; then
    printf 'Refusing to continue without a SHA-1 certificate identity.\n' >&2
    exit 1
fi
{
    printf 'KXCAPTURE_CODESIGN_SHA1=%s\n' "$identity"
    printf 'KXCAPTURE_KEYCHAIN=%s\n' "$keychain"
} >>"$GITHUB_ENV"
printf 'Imported the public test identity into a temporary keychain. Private key files were removed.\n'
