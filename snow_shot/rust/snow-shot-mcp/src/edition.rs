//! Compile-time product identity and the reduced Mini MCP contract.
pub const MINI: bool = cfg!(feature = "mini");
pub const PRODUCT: &str = if MINI { "kxcapture-mini" } else { "kxcapture" };
pub const PRODUCT_NAME: &str = if MINI { "KXCapture Mini" } else { "KXCapture" };
pub const MCP_NAME: &str = if MINI {
    "kxcapture-mini-mcp"
} else {
    "kxcapture-mcp"
};
pub const REGISTRY_NAME: &str = if MINI { "KXCaptureMini" } else { "KXCapture" };
pub const APP_NAME: &str = if MINI { "kxcapture_mini" } else { "kxcapture" };

const TRANSLATION: bool = cfg!(feature = "translation");
const OCR: bool = cfg!(feature = "ocr");

fn translation_method(name: &str) -> bool {
    matches!(
        name,
        "kxcapture_screenshot_translate"
            | "kxcapture_models_list"
            | "kxcapture_models_update"
            | "kxcapture_credentials_set"
            | "kxcapture_translation_catalog"
            | "kxcapture_translation_start"
    )
}

fn ocr_method(name: &str) -> bool {
    matches!(
        name,
        "kxcapture_screenshot_recognize"
            | "kxcapture_document_recognize"
            | "kxcapture_screenshot_edit_recognition"
            | "kxcapture_screenshot_export_recognition"
            | "kxcapture_document_edit_recognition"
    )
}

pub fn method_enabled(name: &str) -> bool {
    if !TRANSLATION && translation_method(name) {
        return false;
    }
    if !OCR && ocr_method(name) {
        return false;
    }
    !MINI || !translation_method(name)
}

pub fn input_enabled(name: &str, input: &serde_json::Value) -> bool {
    if !method_enabled(name) {
        return false;
    }
    if !MINI {
        return true;
    }
    match name {
        "kxcapture_screenshot_recognize" | "kxcapture_document_recognize" => {
            input.get("kind").and_then(serde_json::Value::as_str) == Some("text")
        }
        "kxcapture_pinned_edit" => {
            let action = input.get("action").and_then(serde_json::Value::as_str);
            action != Some("translate")
                && (action != Some("recognize")
                    || input
                        .get("payload")
                        .and_then(|p| p.get("kind"))
                        .and_then(serde_json::Value::as_str)
                        == Some("text"))
        }
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn retained_ids_and_recognition_modes_follow_the_compiled_edition() {
        assert_eq!(method_enabled("kxcapture_document_recognize"), OCR);
        assert_eq!(
            method_enabled("kxcapture_translation_start"),
            !MINI && TRANSLATION
        );
        for kind in ["table", "qr", "latex", "markdown", "html", "document"] {
            assert_eq!(
                input_enabled("kxcapture_document_recognize", &json!({"kind":kind})),
                !MINI
            );
            assert_eq!(
                input_enabled(
                    "kxcapture_pinned_edit",
                    &json!({"action":"recognize","payload":{"kind":kind}})
                ),
                !MINI
            );
        }
        assert!(input_enabled(
            "kxcapture_document_recognize",
            &json!({"kind":"text"})
        ));
    }
}
