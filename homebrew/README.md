# KXCapture Homebrew tap

KXCapture supports Apple Silicon and Intel Macs running macOS 15 or later when
the release provides the corresponding package. KXCapture Mini requires Apple
Silicon. No Homebrew tap is published for KXCapture. Do not install the upstream
Snow Shot cask. After a KXCapture tap exists, its install commands belong here.

If Homebrew reports that `Casks/snow-shot.rb` is missing and offers only
`snow-shot@beta`, the local tap checkout has not received a stable cask.

Beta releases use a separate cask token (`snow-shot@beta`). No KXCapture beta
tap is published.

Both channel tokens install `KXCapture.app` once a cask exists. Quit the app and uninstall the current
cask before switching channels; uninstall preserves user data and signing state.

Project-signed releases after 1.5.8 keep the same publisher identity across DMG,
script, and Homebrew installation. Homebrew verifies the pinned project signature
and copies the bundle unchanged; it needs no user signing key or Keychain access.
First installation or migration from an ad-hoc/local signature may require privacy
consent again. Use the current installer: historical scripts still re-sign apps.
Self-signing does not provide Apple notarization. Legacy installer state and keys
are left untouched for rollback, and uninstall preserves user data.

For an existing manual installation, quit KXCapture and move `KXCapture.app` (or
legacy `snow_shot.app`) out of Applications to a backup folder before installing.
Keep legacy signing state and keys if rollback is needed. Do not use `--force` to overwrite it.
Delete the backup only after confirming the Homebrew installation works.

Use `--appdir="$HOME/Applications"` only after a KXCapture cask exists (create
that directory first). Use the same desktop account for upgrades. Finish captures
and recordings before upgrading or uninstalling.

When the installed cask includes `quit:`, Homebrew asks a running KXCapture to
quit and reopens it after a successful upgrade if it was able to quit the app.
For the first upgrade from a cask without `quit:`, quit KXCapture before upgrading
and open it afterward. If Homebrew reports a quit failure during an upgrade,
quit and reopen the app manually.

The publisher verifies the downloaded DMG against GitHub's asset SHA-256 digest
and the `.sha256` sidecar when present. At least one checksum source is required;
if both exist, both must match. The Homebrew archive always includes a checksum
sidecar for the installer, even when the original release uses only GitHub's digest.
Each architecture has its own reproducible archive. Full casks select the Apple
Silicon or Intel archive automatically when both are present; historical releases
containing only one architecture retain that requirement. Publication refuses to
remove an architecture or omit an edition already present in that release channel.
Keep a GitHub release in draft until all assets have been uploaded and verified;
the Homebrew workflow starts as soon as the release is published. If a partial
upload is rejected, finish uploading and rerun the workflow for the same tag.

Existing archives cannot change at the same version. To repair a cask published
with a missing architecture, manually run **Publish Snow Shot Homebrew cask** with
the same tag and `repair_architectures` enabled. This permits only extending a
single-architecture cask to both architectures when its existing archive checksum,
URL, installer arguments, and all other settings are identical. All archives are
verified before the tap changes. Ordinary publication still rejects same-version
cask changes. The upstream Snow Shot repair document is not a KXCapture feed.
