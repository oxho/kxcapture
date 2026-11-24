#!/bin/bash
# Prove scripts/create-kxcapture-selfsigned-cert.sh on a macOS runner.
# gh secret set is replaced by a PATH shim that writes the values to a temp
# directory. Nothing is sent to GitHub and nothing is written in the repo.
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
secret_dir="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-secret-sink.XXXXXX")"
shim="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-gh-shim.XXXXXX")"
backup="$(mktemp "${TMPDIR:-/tmp}/kxcapture-cert-backup.XXXXXX")"
rm -f "$backup"
keychain="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/kxcapture-cert-test.keychain-db"
backup_keychain="${RUNNER_TEMP:-${TMPDIR:-/tmp}}/kxcapture-backup-cert-test.keychain-db"

cleanup() {
    if [[ -n "${keychain:-}" && -f "$keychain" ]]; then
        security delete-keychain "$keychain" >/dev/null 2>&1 || rm -f "$keychain"
    fi
    if [[ -n "${backup_keychain:-}" && -f "$backup_keychain" ]]; then
        security delete-keychain "$backup_keychain" >/dev/null 2>&1 || rm -f "$backup_keychain"
    fi
    rm -rf "$secret_dir" "$shim" "${dummy_dir:-}"
    if [[ -n "${backup:-}" && -f "$backup" ]]; then
        rm -P "$backup" >/dev/null 2>&1 || rm -f "$backup"
    fi
    if [[ -n "${backup:-}" && -f "${backup}.password" ]]; then
        rm -P "${backup}.password" >/dev/null 2>&1 || rm -f "${backup}.password"
    fi
}
trap cleanup EXIT

cat >"$shim/gh" <<'EOF'
#!/bin/bash
set -euo pipefail
if [[ "${1:-}" == auth ]]; then
    exit 0
fi
if [[ "${1:-}" == api ]]; then
    printf 'oxho/kxcapture\n'
    exit 0
fi
if [[ "${1:-}" == secret && "${2:-}" == set ]]; then
    cat > "${KXCAPTURE_SECRET_DIR}/${3}"
    exit 0
fi
printf 'unexpected gh invocation\n' >&2
exit 1
EOF
chmod +x "$shim/gh"

export PATH="$shim:$PATH"
export KXCAPTURE_SECRET_DIR="$secret_dir"
script_output="$(bash "$root/scripts/create-kxcapture-selfsigned-cert.sh" --keep-backup "$backup")"

test -s "$secret_dir/KXCAPTURE_MAC_CERT_P12_BASE64"
test -s "$secret_dir/KXCAPTURE_MAC_CERT_P12_PASSWORD"
test -s "$backup"
test -s "${backup}.password"
password_mode="$(stat -f '%Lp' "${backup}.password")"
if [[ "$password_mode" != "600" ]]; then
    printf 'backup password file mode is %s, expected 600\n' "$password_mode" >&2
    exit 1
fi
if ! cmp -s "$secret_dir/KXCAPTURE_MAC_CERT_P12_PASSWORD" "${backup}.password"; then
    printf 'backup password file does not match the value stored for the Actions secret\n' >&2
    exit 1
fi
if printf '%s\n' "$script_output" | grep -F -f "${backup}.password" >/dev/null; then
    printf 'certificate script printed the password\n' >&2
    exit 1
fi
printf '%s\n' "$script_output"

export GITHUB_ENV="$secret_dir/github.env"
export KXCAPTURE_KEYCHAIN="$keychain"
KXCAPTURE_MAC_CERT_P12_BASE64="$(cat "$secret_dir/KXCAPTURE_MAC_CERT_P12_BASE64")" \
    KXCAPTURE_MAC_CERT_P12_PASSWORD="$(cat "$secret_dir/KXCAPTURE_MAC_CERT_P12_PASSWORD")" \
    bash "$root/scripts/ci/macos-import-signing-cert.sh"
# shellcheck disable=SC1090
source "$GITHUB_ENV"

dummy_dir="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-dummy.XXXXXX")"
printf 'int main(void){return 0;}\n' >"$dummy_dir/dummy.c"
clang -o "$dummy_dir/dummy" "$dummy_dir/dummy.c"
codesign --force --sign "$KXCAPTURE_CODESIGN_SHA1" "$dummy_dir/dummy"
codesign --verify --verbose=2 "$dummy_dir/dummy"
security find-certificate -c 'KXCapture Self-Signed' -p "$keychain" \
    | /usr/bin/openssl x509 -noout -checkend $((3649 * 24 * 3600))
security find-certificate -c 'KXCapture Self-Signed' -p "$keychain" \
    | /usr/bin/openssl x509 -noout -subject -dates -fingerprint -sha256

# Import the backup PKCS#12 with the password file, independent of the secret sink.
backup_password="$(cat "${backup}.password")"
security create-keychain -p "$backup_password" "$backup_keychain"
security set-keychain-settings -lut 28800 "$backup_keychain"
security unlock-keychain -p "$backup_password" "$backup_keychain"
security import "$backup" -k "$backup_keychain" -P "$backup_password" -T /usr/bin/codesign -T /usr/bin/security >/dev/null
security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k "$backup_password" "$backup_keychain" >/dev/null
security list-keychains -d user -s "$backup_keychain" "$keychain" login.keychain-db
backup_sha1="$(security find-certificate -c 'KXCapture Self-Signed' -p "$backup_keychain" | /usr/bin/openssl x509 -noout -fingerprint -sha1)"
backup_identity="$(printf '%s\n' "$backup_sha1" | awk -F= '{print $2}' | tr -d ':' | tr '[:lower:]' '[:upper:]')"
unset backup_password
printf 'int main(void){return 0;}\n' >"$dummy_dir/backup.c"
clang -o "$dummy_dir/backup" "$dummy_dir/backup.c"
codesign --force --sign "$backup_identity" "$dummy_dir/backup"
codesign --verify --verbose=2 "$dummy_dir/backup"
printf 'Certificate script test passed: codesign verify succeeded, validity is at least 3649 days, and the backup PKCS#12 imports with its password file.\n'
