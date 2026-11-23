//! Product identities are selected at compile time, never by downloaded metadata.
pub const MINI: bool = cfg!(feature = "mini");
pub const PLATFORM: &str = if cfg!(target_arch = "aarch64") {
    "windows-arm64"
} else {
    "windows-x64"
};

pub fn validate_platform(platform: &str) -> crate::Result<()> {
    crate::error::require(
        matches!(platform, "windows-x64" | "windows-arm64"),
        "unsupported_update_release",
        "Unsupported update release",
    )
}

pub fn package_prefix(platform: &str) -> String {
    format!("setup/{PRODUCT}_{platform}-")
}
macro_rules! identity {
    ($name:ident, $full:literal, $mini:literal) => {
        pub const $name: &str = if MINI { $mini } else { $full };
    };
}
identity!(PRODUCT, "kxcapture", "kxcapture-mini");
identity!(PRODUCT_NAME, "KXCapture", "KXCapture Mini");
identity!(REGISTRY_NAME, "KXCapture", "KXCaptureMini");
identity!(SHELL_VERB, "KXCapture.Edit", "KXCaptureMini.Edit");
identity!(APP_PATH, "bin/kxcapture.exe", "bin/kxcapture_mini.exe");
identity!(
    UPDATER_PATH,
    "bin/kxcapture-updater.exe",
    "bin/kxcapture-mini-updater.exe"
);
identity!(
    UPDATER_NONWINDOWS_PATH,
    "bin/kxcapture-updater",
    "bin/kxcapture-mini-updater"
);
identity!(UPDATER_NAME, "kxcapture-updater", "kxcapture-mini-updater");
identity!(
    INSTALLATION_RECORD,
    "kxcapture-installation.json",
    "kxcapture-mini-installation.json"
);
identity!(UPDATE_WORK, ".kxcapture-update", ".kxcapture-mini-update");
identity!(SHARE_PREFIX, "share/kxcapture/", "share/kxcapture-mini/");
pub const PACKAGE_PREFIX: &str = match (MINI, cfg!(target_arch = "aarch64")) {
    (false, false) => "setup/kxcapture_windows-x64-",
    (true, false) => "setup/kxcapture-mini_windows-x64-",
    (false, true) => "setup/kxcapture_windows-arm64-",
    (true, true) => "setup/kxcapture-mini_windows-arm64-",
};
pub const FEED_NAME: &str = match (MINI, cfg!(target_arch = "aarch64")) {
    (false, false) => "latest-version.json",
    (true, false) => "latest-version-mini.json",
    (false, true) => "latest-version-windows-arm64.json",
    (true, true) => "latest-version-mini-windows-arm64.json",
};
identity!(TEMP_PREFIX, "kxcapture-updater-", "kxcapture-mini-updater-");
identity!(
    MARKER_PATH,
    "bin/__data_directory",
    "bin/__mini_data_directory"
);
identity!(
    INSTALL_KEY,
    "Software\\oxho\\KXCapture",
    "Software\\oxho\\KXCaptureMini"
);
identity!(
    UNINSTALL_KEY,
    "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\KXCapture",
    "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\KXCaptureMini"
);

pub fn product_matches(value: Option<&serde_json::Value>) -> bool {
    match value {
        None => !MINI,
        Some(value) => value.as_str() == Some(PRODUCT),
    }
}

pub fn installation_variant(variant: &str) -> bool {
    matches!(variant, "online" | "portable") || (!MINI && variant == "offline")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn product_binding_accepts_only_this_product_and_legacy_full_records() {
        assert!(product_matches(Some(&json!(PRODUCT))));
        assert!(!product_matches(Some(&json!(if MINI {
            "snow-shot"
        } else {
            "snow-shot-mini"
        }))));
        assert!(!product_matches(Some(&json!(null))));
        assert_eq!(product_matches(None), !MINI);
        assert_eq!(installation_variant("offline"), !MINI);
        assert!(installation_variant("online"));
        assert!(installation_variant("portable"));
    }
}
