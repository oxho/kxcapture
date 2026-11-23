cask "snow-shot" do
  arch arm: "arm64", intel: "x86_64"

  version "1.3.4"
  sha256 arm:   "0000000000000000000000000000000000000000000000000000000000000000",
         intel: "1111111111111111111111111111111111111111111111111111111111111111"

  url ""
  name "KXCapture"
  desc "Screenshot and screen recording application"
  homepage ""

  depends_on macos: :sequoia

  app "KXCapture.app"

  # Third-party Ruby hooks preserve the desktop user's HOME and Keychain access.
  preflight do
    system_command "/bin/bash",
                   args:         [staged_path.join("prepare-snow-shot-homebrew.sh"),
                                  staged_path.join("snow-shot-#{version}-macos-#{arch}.dmg"),
                                  staged_path.join("KXCapture.app")],
                   must_succeed: true,
                   print_stdout: true,
                   print_stderr: true
  end

  caveats <<~EOS
    KXCapture reuses a signing identity in your login Keychain. The first install
    may request Keychain access. Grant Screen Recording and Accessibility when
    macOS requests them. Local signing does not provide Apple notarization.
    Keep ~/Library/Application Support/KXCapture/Installer and its Keychain
    identity across upgrades and reinstalls. Use the same installing account.
  EOS
end
