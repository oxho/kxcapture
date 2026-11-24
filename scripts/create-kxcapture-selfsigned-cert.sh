#!/bin/bash
# Create a 10-year KXCapture code-signing certificate and store it in GitHub
# Actions secrets. Run this once on a Mac. The private key, PKCS#12, and
# password are never printed and never written inside the repository.
set -euo pipefail

repo="oxho/kxcapture"
secret_p12="KXCAPTURE_MAC_CERT_P12_BASE64"
secret_password="KXCAPTURE_MAC_CERT_P12_PASSWORD"
backup=""

usage() {
    printf 'Usage: %s [--keep-backup <path-outside-the-repo>]\n' "$0" >&2
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --keep-backup)
            if [[ $# -lt 2 || -z "${2:-}" ]]; then
                usage
                exit 2
            fi
            backup="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            usage
            exit 2
            ;;
    esac
done

if ! command -v gh >/dev/null 2>&1; then
    printf 'gh is not installed. Install GitHub CLI and run gh auth login with an account that can set Actions secrets on %s.\n' "$repo" >&2
    exit 1
fi
if ! gh auth status >/dev/null 2>&1; then
    printf 'gh is not logged in. Run gh auth login with an account that can set Actions secrets on %s.\n' "$repo" >&2
    exit 1
fi
if ! gh api "repos/${repo}" --jq .full_name >/dev/null 2>&1; then
    printf 'gh is logged in but cannot access %s. Use an account that can set Actions secrets on that repository.\n' "$repo" >&2
    exit 1
fi

if [[ -n "$backup" ]]; then
    case "$backup" in
        /*.p12|/*.pfx) ;;
        /*)
            printf 'The --keep-backup path must end in .p12 so macOS security import recognizes the file.\n' >&2
            exit 2
            ;;
        *)
            printf 'The --keep-backup path must be absolute and outside the repository.\n' >&2
            exit 2
            ;;
    esac
    if git rev-parse --show-toplevel >/dev/null 2>&1; then
        repo_root="$(git rev-parse --show-toplevel)"
        backup_dir="$(cd "$(dirname "$backup")" && pwd)"
        case "$backup_dir" in
            "$repo_root"|"$repo_root"/*)
                printf 'Refusing to write the certificate backup inside the repository.\n' >&2
                exit 2
                ;;
        esac
    fi
fi

choose_openssl() {
    if [[ -n "${KXCAPTURE_OPENSSL:-}" ]]; then
        printf '%s\n' "$KXCAPTURE_OPENSSL"
        return
    fi
    if [[ -x /usr/bin/openssl ]] && /usr/bin/openssl version 2>/dev/null | grep -q LibreSSL; then
        printf '%s\n' /usr/bin/openssl
        return
    fi
    command -v openssl
}

openssl_bin="$(choose_openssl)"
openssl_version="$("$openssl_bin" version)"
pkcs12_legacy=""
if [[ "$openssl_version" == OpenSSL\ 3* ]]; then
    # Homebrew OpenSSL 3 writes a PKCS#12 that macOS security import rejects
    # unless the legacy MAC algorithms are requested.
    pkcs12_legacy="-legacy"
fi

if git rev-parse --show-toplevel >/dev/null 2>&1; then
    repo_root="$(git rev-parse --show-toplevel)"
    temp_root="$(cd "${TMPDIR:-/tmp}" && pwd)"
    case "$temp_root" in
        "$repo_root"|"$repo_root"/*)
            export TMPDIR=/tmp
            ;;
    esac
fi
work="$(mktemp -d "${TMPDIR:-/tmp}/kxcapture-selfsigned.XXXXXX")"
chmod 700 "$work"
secure_rm() {
    local path="$1"
    if [[ -f "$path" ]]; then
        rm -P "$path" >/dev/null 2>&1 || rm -f "$path"
    fi
}
cleanup() {
    if [[ -d "$work" ]]; then
        find "$work" -type f -exec rm -P {} + >/dev/null 2>&1 || true
        rm -rf "$work"
    fi
}
trap cleanup EXIT

config="$work/certificate.cnf"
cat >"$config" <<'EOF'
[req]
prompt = no
distinguished_name = subject
x509_extensions = signing
[subject]
CN = KXCapture Self-Signed
[signing]
keyUsage = critical,digitalSignature
extendedKeyUsage = codeSigning
EOF

"$openssl_bin" req -new -newkey rsa:3072 -nodes -x509 -days 3650 \
    -config "$config" \
    -keyout "$work/private.pem" \
    -out "$work/certificate.pem" >/dev/null

password="$("$openssl_bin" rand -hex 32)"
# The password is passed in the environment. It is not placed on the openssl
# command line, where ps would show it.
KX_P12_PASS="$password" "$openssl_bin" pkcs12 -export \
    $pkcs12_legacy \
    -inkey "$work/private.pem" \
    -in "$work/certificate.pem" \
    -out "$work/identity.p12" \
    -passout env:KX_P12_PASS >/dev/null
unset KX_P12_PASS
secure_rm "$work/private.pem"

if [[ -n "$backup" ]]; then
    install -m 600 "$work/identity.p12" "$backup"
    (umask 077; printf '%s' "$password" >"${backup}.password")
    chmod 600 "${backup}.password"
fi

p12_b64="$("$openssl_bin" base64 -in "$work/identity.p12" | tr -d '\n')"
printf '%s' "$p12_b64" | gh secret set "$secret_p12" -R "$repo" >/dev/null
printf '%s' "$password" | gh secret set "$secret_password" -R "$repo" >/dev/null
unset p12_b64

fingerprint="$("$openssl_bin" x509 -in "$work/certificate.pem" -noout -fingerprint -sha256)"
not_after="$("$openssl_bin" x509 -in "$work/certificate.pem" -noout -enddate)"
printf '%s\n' "$fingerprint"
printf '%s\n' "$not_after"
if [[ -n "$backup" ]]; then
    printf 'backup p12: %s\n' "$backup"
    printf 'backup password: %s\n' "${backup}.password"
fi
unset password
