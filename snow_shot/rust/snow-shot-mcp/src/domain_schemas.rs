//! Application-domain contracts. Qt validates registry keys and resource invariants.
use super::*;

macro_rules! input {
    ($name:ident { $($(#[$attribute:meta])* $field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Deserialize, JsonSchema)]
        #[serde(deny_unknown_fields)]
        struct $name { $($(#[$attribute])* $field: $ty),* }
    };
}
macro_rules! choices {
    ($name:ident { $($(#[$attribute:meta])* $variant:ident),* $(,)? }) => {
        #[derive(Deserialize, JsonSchema)]
        #[serde(rename_all = "snake_case")]
        enum $name { $($(#[$attribute])* $variant),* }
    };
}
macro_rules! limited_integer {
    ($name:ident, $minimum:expr, $maximum:expr) => {
        struct $name(u32);
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error> {
                let value=u32::deserialize(deserializer)?;
                if ($minimum..=$maximum).contains(&value) {Ok(Self(value))}
                else {Err(serde::de::Error::custom(concat!(stringify!($name)," is outside the supported range")))}
            }
        }
        impl JsonSchema for $name {
            fn schema_name()->std::borrow::Cow<'static,str>{ stringify!($name).into() }
            fn json_schema(_: &mut schemars::SchemaGenerator)->schemars::Schema {
                schemars::json_schema!({"type":"integer","minimum":$minimum,"maximum":$maximum})
            }
        }
    };
}
macro_rules! integer_choices {
    ($name:ident, [$($allowed:expr),*]) => {
        struct $name(u32);
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error> {
                let value=u32::deserialize(deserializer)?;
                if [$($allowed),*].contains(&value) {Ok(Self(value))}
                else {Err(serde::de::Error::custom(concat!("Unsupported ", stringify!($name))))}
            }
        }
        impl JsonSchema for $name {
            fn schema_name()->std::borrow::Cow<'static,str>{ stringify!($name).into() }
            fn json_schema(_: &mut schemars::SchemaGenerator)->schemars::Schema {
                schemars::json_schema!({"type":"integer","enum":[$($allowed),*]})
            }
        }
    };
}
struct AudioGainDb(i32);
impl<'de> Deserialize<'de> for AudioGainDb {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        if (-24..=24).contains(&value) {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom(
                "Audio gain must be between -24 and 24 dB",
            ))
        }
    }
}
impl JsonSchema for AudioGainDb {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "AudioGainDb".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"integer","minimum":-24,"maximum":24})
    }
}
limited_integer!(DelaySeconds, 0, 10);
limited_integer!(HistoryPageSize, 1, 200);
limited_integer!(TrailDuration, 100, 2000);
limited_integer!(KeyboardSize, 32, 128);
limited_integer!(WebcamSize, 50, 500);
limited_integer!(MouseHighlightRadius, 8, 128);
limited_integer!(Percentage, 0, 100);
limited_integer!(ArtifactChunkSize, 1, 262144);
limited_integer!(ExportQuality, 1, 100);
integer_choices!(FrameRate, [5, 10, 15, 24, 30, 60, 120, 83]);
integer_choices!(AnimatedFrameRate, [5, 10, 15, 24]);
struct DocumentScale(f64);
impl<'de> Deserialize<'de> for DocumentScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = f64::deserialize(deserializer)?;
        if (0.1..=4.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(serde::de::Error::custom(
                "Document scale must be between 0.1 and 4",
            ))
        }
    }
}
impl JsonSchema for DocumentScale {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DocumentScale".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"number","minimum":0.1,"maximum":4})
    }
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Revision<T> {
    expected_revision: u64,
    idempotency_key: Option<String>,
    #[serde(flatten)]
    options: T,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DocumentMutation<T> {
    document_id: String,
    expected_revision: u64,
    idempotency_key: Option<String>,
    #[serde(flatten)]
    options: T,
}
#[derive(Deserialize, JsonSchema)]
#[serde(untagged)]
#[schemars(extend("type" = "object"))]
enum DocumentToolStyleMutation {
    Magnifier(DocumentMutation<MagnifierToolStyle>),
    Standard(Box<DocumentMutation<StandardToolStyle>>),
    BrushEraser(DocumentMutation<BrushEraserToolStyle>),
    Angle(DocumentMutation<AngleToolStyle>),
    Bracket(DocumentMutation<BracketToolStyle>),
}
input!(Section { section: Option<String> });
choices!(AppAction {
    ShowMain,
    ShowSettings,
    ShowHistory,
    ShowPinned,
    #[cfg(not(feature = "mini"))]
    ShowTranslation,
    Restart,
    Quit
});
input!(AppActionInput { action: AppAction, page_id: Option<String>, section_id: Option<String> });
input!(SettingsUpdate { values: Map<String, Value> });
input!(SettingsReset {
    page_id: String,
    section_id: String
});
input!(SettingsAction { field_id: String, path: Option<String> });
#[cfg(not(feature = "mini"))]
choices!(ModelProvider {
    Openai,
    Qwen,
    Deepseek,
    Doubao
});
#[cfg(not(feature = "mini"))]
input!(Model { id: String, name: String, base_url: String, model: String, supports_vision: Option<bool>, supports_reasoning: Option<bool>, api_key: Option<String>, credential_set: Option<bool>, provider: Option<ModelProvider> });
#[cfg(not(feature = "mini"))]
input!(ModelsUpdate { models: Vec<Model> });
#[cfg(not(feature = "mini"))]
input!(Credential {
    provider: String,
    secret: String
});
input!(HistoryList { cursor: Option<String>, limit: Option<HistoryPageSize>, query: Option<String> });
input!(HistoryGet { history_id: String, include_image: Option<bool> });
input!(HistoryId { history_id: String });
choices!(HistoryAction { Edit, Pin });
input!(HistoryActionInput {
    history_id: String,
    action: HistoryAction
});
input!(FilePath { path: String });
choices!(CleanupKind {
    ThumbnailCache,
    TemporaryFiles,
    History,
    Pinned
});
input!(Cleanup { kind: CleanupKind });
input!(Permission { permission: String, open_settings: Option<bool> });
choices!(UpdateAction {
    Check,
    Download,
    Cancel,
    Apply
});
input!(UpdateInput {
    action: UpdateAction
});
choices!(TemplateKind { Drawing, Watermark });
input!(TemplateList { kind: TemplateKind });
input!(Template { name: String, payload: Option<Map<String, Value>>, text: Option<String> });
input!(TemplateUpdate { kind: TemplateKind, templates: Vec<Template> });
#[cfg(not(feature = "mini"))]
choices!(TranslationSource { Selection });
#[cfg(not(feature = "mini"))]
input!(TranslationStart { texts: Option<Vec<String>>, source: Option<TranslationSource>, source_language: Option<String>, target_language: Option<String>, model_id: Option<String>, retry_job_id: Option<String>, idempotency_key: Option<String> });

choices!(DocumentSource {
    File,
    History,
    Pinned,
    Clipboard,
    Capture,
    Text,
    Html
});
choices!(DocumentTarget {
    CurrentMonitor,
    FocusedWindow,
    Monitor
});
input!(DocumentOpen { path: Option<String>, source: Option<DocumentSource>, source_id: Option<String>, target: Option<DocumentTarget>, monitor_id: Option<String>, scale: Option<DocumentScale>, capture_cursor: Option<bool>, delay_seconds: Option<DelaySeconds>, text: Option<String>, html: Option<String>, idempotency_key: Option<String>, as_job: Option<bool> });
input!(DocumentRecapture { path: Option<String>, source: Option<DocumentSource>, source_id: Option<String>, target: Option<DocumentTarget>, monitor_id: Option<String>, scale: Option<DocumentScale>, capture_cursor: Option<bool>, delay_seconds: Option<DelaySeconds>, text: Option<String>, html: Option<String> });
input!(DocumentId {
    document_id: String
});
input!(DocumentPoint { point: [f64; 2] });
choices!(DocumentCanvasTool {
    SpeechBubble,
    Magnifier,
    Distance,
    Angle,
    Bracket,
    Select,
    Rectangle,
    Arrow,
    Line,
    Freehand,
    RectangleHighlight,
    PenHighlight,
    Eraser,
    RectangleFilter,
    PenFilter,
    Text,
    SerialNumber,
    Watermark,
    Spotlight,
    AutoFilter,
    RectangleEraser,
    BrushEraser
});
input!(DocumentTool {
    tool: DocumentCanvasTool
});
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DocumentSave {
    path: String,
    scale: Option<DocumentScale>,
    format: Option<Format>,
    quality: Option<ExportQuality>,
    compression_level: Option<CompressionLevel>,
    pdf_page_size: Option<PdfPageSize>,
    #[schemars(length(max = 1024))]
    pdf_title: Option<String>,
}
choices!(DocumentElementAction {
    Select,
    Order,
    Align,
    Opacity,
    Duplicate,
    Delete,
    DeleteAll,
    AdjustSerialNumbers,
    CreateSerialText,
    ErasePath
});
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct DocumentElementEdit {
    action: DocumentElementAction,
    id: Option<ElementId>,
    order: Option<Order>,
    alignment: Option<Alignment>,
    opacity: Option<f64>,
    delta: Option<i32>,
    #[schemars(length(min = 1, max = 8192))]
    points: Option<Vec<[f64; 2]>>,
}
input!(DocumentRecognitionEdit {
    expected_recognition_revision: u64,
    action: RecognitionAction,
    text: Option<String>,
    #[cfg(not(feature = "mini"))]
    value: Option<String>,
    #[cfg(not(feature = "mini"))]
    range: Option<[u32;4]>,
    #[cfg(not(feature = "mini"))]
    row: Option<u32>,
    #[cfg(not(feature = "mini"))]
    column: Option<u32>,
    #[cfg(not(feature = "mini"))]
    enabled: Option<bool>
});
choices!(OriginalContentFormat { Json, Text, Html });
input!(OriginalContentOutput { output: RecognitionOutput, format: Option<OriginalContentFormat>, path: Option<String> });
input!(DocumentSelection { operation: Option<RegionOperation>, r#type: Option<RegionType>, bounds: Option<[f64; 4]>, points: Option<Vec<[f64; 2]>> });
input!(DocumentRecognize {
    kind: RecognitionKind
});
input!(JobId { job_id: String });
input!(ArtifactRead {artifact_id:String,offset:Option<u64>,max_bytes:Option<ArtifactChunkSize>});
input!(ArtifactRelease {artifact_id:String,idempotency_key:Option<String>});

choices!(RecordingFormat {
    Mp4,
    Gif,
    Apng,
    Webp
});
choices!(Encoder {
    H264,
    H265,
    H264Hw,
    H265Hw
});
#[derive(Deserialize, JsonSchema)]
enum Clarity {
    #[serde(rename = "4k")]
    Uhd,
    #[serde(rename = "2k")]
    Qhd,
    #[serde(rename = "1080p")]
    FullHd,
    #[serde(rename = "720p")]
    Hd,
    #[serde(rename = "480p")]
    Sd,
}
#[derive(Deserialize, JsonSchema)]
enum AnimatedClarity {
    #[serde(rename = "1080p")]
    FullHd,
    #[serde(rename = "720p")]
    Hd,
    #[serde(rename = "480p")]
    Sd,
}
choices!(EncodingPreset {
    Ultrafast,
    Veryfast,
    Medium,
    Veryslow,
    Placebo
});
choices!(PostProcessingEffect {
    ProgressBar,
    PlaybackTime
});
choices!(WebcamShape { Square, Circle });
// Missing overrides use application preferences. A supplied override must have
// the declared value type; JSON null is not a request to reset that preference.
#[derive(JsonSchema)]
#[schemars(with = "T", inline)]
struct SuppliedRecordingOption<T>(Option<T>);
impl<T> Default for SuppliedRecordingOption<T> {
    fn default() -> Self {
        Self(None)
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for SuppliedRecordingOption<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(|value| Self(Some(value)))
    }
}
macro_rules! recording_options {
    ($($field:ident: $ty:ty),* $(,)?) => {
        #[derive(Deserialize, JsonSchema)]
        #[serde(deny_unknown_fields)]
        struct RecordingOptions {
            $(#[serde(default)] $field: SuppliedRecordingOption<$ty>),*
        }
    };
}
recording_options! {
    path: String,
    format: RecordingFormat,
    quality: Percentage,
    start_delay_seconds: DelaySeconds,
    microphone: bool,
    system_audio: bool,
    system_audio_gain_db: AudioGainDb,
    microphone_gain_db: AudioGainDb,
    separate_audio_tracks: bool,
    frame_rate: FrameRate,
    animated_frame_rate: AnimatedFrameRate,
    clarity: Clarity,
    animated_clarity: AnimatedClarity,
    encoder: Encoder,
    encoding_preset: EncodingPreset,
    r#loop: bool,
    capture_toolbar: bool,
    post_processing: bool,
    post_processing_effect: PostProcessingEffect,
    progress_bar_color: String,
    show_cursor: bool,
    show_keyboard: bool,
    record_number_keys: bool,
    record_letters_and_symbols: bool,
    record_control_keys: bool,
    preserve_keyboard_shortcuts: bool,
    mouse_highlight: bool,
    webcam: bool,
    webcam_size: WebcamSize,
    webcam_shape: WebcamShape,
    record_mouse_clicks: bool,
    record_mouse_scrolling: bool,
    mouse_trail_duration_ms: TrailDuration,
    keyboard_size: KeyboardSize,
    keyboard_background: String,
    keyboard_foreground: String,
    mouse_trail: String,
    mouse_click: String,
    mouse_highlight_color: String,
    mouse_highlight_radius: MouseHighlightRadius,
}
input!(RecordingStart { region: [f64; 4], options: Option<RecordingOptions>, idempotency_key: Option<String> });
choices!(RecordingAction {
    Pause,
    Resume,
    Stop,
    Cancel,
    CancelRender,
    Copy,
    Close,
    Retry,
    KeepSource,
    Discard,
    Annotations,
    Undo,
    Redo,
    Reset
});
input!(RecordingControl { recording_id: String, action: RecordingAction, payload: Option<Annotations> });
input!(PinnedId { id: String });
input!(PinnedList { offset: Option<u32>, limit: Option<HistoryPageSize> });
#[derive(Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum PinnedSource {
    File { path: String },
    Clipboard,
    Text { text: String },
    Html { html: String },
}
input!(PinnedCreate { source: PinnedSource, group_id: Option<String>, idempotency_key: Option<String> });
input!(PinnedReplace {
    id: String,
    source: PinnedSource
});
choices!(Rotation {
    Clockwise,
    Counterclockwise,
    Reset
});
choices!(Flip {
    Horizontal,
    Vertical
});
input!(PinnedProperties { geometry: Option<[f64; 4]>, scale_percent: Option<f64>, opacity_percent: Option<Percentage>, click_through: Option<bool>, click_through_opacity_percent: Option<Percentage>, always_on_top: Option<bool>, show_border: Option<bool>, show_shadow: Option<bool>, thumbnail: Option<bool>, hide_to_top: Option<bool>, rotation: Option<Rotation>, flip: Option<Flip> });
input!(PinnedUpdate {
    id: String,
    properties: PinnedProperties
});
choices!(PinnedAction {
    Show,
    Hide,
    Close,
    Destroy,
    RestoreLast,
    ShowAll,
    HideOthers,
    CloseOthers,
    CloseAll
});
input!(PinnedActionInput { action: PinnedAction, id: Option<String> });
choices!(PinnedOutput { Copy, Save });
input!(PinnedExport { id: String, output: PinnedOutput, path: Option<String>, original: Option<bool> });
choices!(PinnedEditAction {
    Annotations,
    Undo,
    Redo,
    Reset,
    Recognize,
    #[cfg(not(feature = "mini"))]
    Translate,
    RecognitionEdit,
    Duplicate,
    Delete,
    Order,
    Align,
    Opacity,
    SerialText,
    SerialAdjust,
    TemplateInsert,
    TemplateExport,
    AutoFilter,
    Tool,
    ToolStyle,
    Editing
});
input!(Editing { enabled: bool });
input!(Duplicate { offset:Option<[f64;2]> });
input!(Ordering { order: Order });
input!(Aligning {
    alignment: Alignment
});
input!(Opacity { opacity: f64 });
input!(SerialAdjust { delta: i32 });
input!(TemplateInsert {
    payload: String,
    center: [f64; 2]
});
choices!(FilterCategory {
    Text,
    TextInBox,
    Image,
    Avatar,
    Icon,
    MessageBox,
    TextBlock
});
input!(PinnedAutoFilter { categories:Vec<FilterCategory> });
#[derive(Deserialize, JsonSchema)]
#[serde(untagged)]
enum PinnedPayload {
    Annotations(Annotations),
    Recognize(DocumentRecognize),
    RecognitionEdit(EditRecognition),
    Duplicate(Duplicate),
    Ordering(Ordering),
    Aligning(Aligning),
    Opacity(Opacity),
    SerialAdjust(SerialAdjust),
    TemplateInsert(TemplateInsert),
    AutoFilter(PinnedAutoFilter),
    Tool(DocumentTool),
    ToolStyle(Box<ToolStyle>),
    Editing(Editing),
}
input!(PinnedEdit { id: String, action: PinnedEditAction, payload: Option<PinnedPayload> });
choices!(GroupAction {
    Create,
    Activate,
    Move,
    Delete,
    DeleteEmpty
});
input!(GroupUpdate { action: GroupAction, name: Option<String>, group_id: Option<String>, id: Option<String> });

pub const TOOLS: &[(&str, &str, bool)] = &[
    (
        "kxcapture_artifact_list",
        "List owned retained artifacts with MIME, size, digest and expiry metadata.",
        true,
    ),
    (
        "kxcapture_artifact_read",
        "Read an owned artifact in bounded base64 chunks; continue with next_offset until eof.",
        true,
    ),
    (
        "kxcapture_artifact_release",
        "Release an owned retained artifact.",
        false,
    ),
    (
        "kxcapture_app_status",
        "Read application status and available feature domains.",
        true,
    ),
    (
        "kxcapture_app_displays",
        "List capture displays and their coordinate mappings.",
        true,
    ),
    (
        "kxcapture_app_action",
        "Open an application page, restart, or quit KXCapture.",
        false,
    ),
    (
        "kxcapture_settings_get",
        "Read settings with typed field metadata; credentials are never returned.",
        true,
    ),
    (
        "kxcapture_settings_update",
        "Update settings by field ID with an expected settings revision.",
        false,
    ),
    (
        "kxcapture_settings_reset",
        "Reset the specified settings section.",
        false,
    ),
    (
        "kxcapture_settings_action",
        "Run a settings action with explicit arguments.",
        false,
    ),
    #[cfg(not(feature = "mini"))]
    (
        "kxcapture_models_list",
        "Read configured model metadata without credentials.",
        true,
    ),
    #[cfg(not(feature = "mini"))]
    (
        "kxcapture_models_update",
        "Update model definitions, retaining omitted credentials.",
        false,
    ),
    #[cfg(not(feature = "mini"))]
    (
        "kxcapture_credentials_set",
        "Replace a model credential without returning its value.",
        false,
    ),
    (
        "kxcapture_history_list",
        "Search screenshot history using a bounded page and opaque cursor.",
        true,
    ),
    (
        "kxcapture_history_get",
        "Read a history entry, optionally including its image.",
        true,
    ),
    (
        "kxcapture_history_delete",
        "Delete a history entry at the expected history revision.",
        false,
    ),
    (
        "kxcapture_history_clear",
        "Clear screenshot history at the expected history revision.",
        false,
    ),
    (
        "kxcapture_history_action",
        "Open a history image in the editor or pin it.",
        false,
    ),
    (
        "kxcapture_configuration_export",
        "Export configuration to an explicit absolute path.",
        false,
    ),
    (
        "kxcapture_configuration_import",
        "Import configuration from an explicit absolute path.",
        false,
    ),
    (
        "kxcapture_storage_status",
        "Read cache and storage usage.",
        true,
    ),
    (
        "kxcapture_storage_cleanup",
        "Clean an explicitly selected storage category.",
        false,
    ),
    (
        "kxcapture_permissions_get",
        "Read native capture and accessibility permission state.",
        true,
    ),
    (
        "kxcapture_permissions_request",
        "Request an OS permission or open its settings page.",
        false,
    ),
    (
        "kxcapture_updates_status",
        "Read application update state.",
        true,
    ),
    (
        "kxcapture_updates_action",
        "Check, download, cancel, or apply an application update.",
        false,
    ),
    (
        "kxcapture_templates_list",
        "List drawing or watermark templates.",
        true,
    ),
    (
        "kxcapture_templates_update",
        "Replace a validated drawing or watermark template collection.",
        false,
    ),
    #[cfg(not(feature = "mini"))]
    (
        "kxcapture_translation_catalog",
        "Read translation models, languages, and preferences without credentials.",
        true,
    ),
    #[cfg(not(feature = "mini"))]
    (
        "kxcapture_translation_start",
        "Translate explicit text using configured providers and return an owned job handle.",
        false,
    ),
    (
        "kxcapture_document_open",
        "Open an isolated background image document; file is the default source.",
        false,
    ),
    (
        "kxcapture_document_list",
        "List background documents owned by this client.",
        true,
    ),
    (
        "kxcapture_document_state",
        "Read an owned background document and its revision.",
        true,
    ),
    (
        "kxcapture_document_clone",
        "Clone an owned background document.",
        false,
    ),
    (
        "kxcapture_document_apply_annotations",
        "Apply one typed annotation transaction to a background document.",
        false,
    ),
    (
        "kxcapture_document_set_tool",
        "Select the active background document canvas tool.",
        false,
    ),
    (
        "kxcapture_document_set_tool_style",
        "Set background document drawing defaults or selected styles.",
        false,
    ),
    (
        "kxcapture_document_edit_elements",
        "Select, duplicate, order, align or edit background document elements.",
        false,
    ),
    (
        "kxcapture_document_draw_template",
        "Insert or export a background document drawing template.",
        false,
    ),
    (
        "kxcapture_document_sample_color",
        "Read the rendered color at a canvas point.",
        true,
    ),
    (
        "kxcapture_document_recapture",
        "Replace the source of an owned document, preserving valid annotations and selection.",
        false,
    ),
    (
        "kxcapture_document_present",
        "Explicitly present an owned document in the visible editor.",
        false,
    ),
    (
        "kxcapture_document_set_selection",
        "Change a background document selection in canvas coordinates.",
        false,
    ),
    (
        "kxcapture_document_undo",
        "Undo the last background document edit.",
        false,
    ),
    (
        "kxcapture_document_redo",
        "Redo a background document edit.",
        false,
    ),
    (
        "kxcapture_document_render",
        "Render an immutable snapshot of a background document.",
        true,
    ),
    (
        "kxcapture_document_copy",
        "Copy a background document snapshot to the clipboard.",
        false,
    ),
    (
        "kxcapture_document_pin",
        "Pin a background document snapshot.",
        false,
    ),
    (
        "kxcapture_document_set_selection_style",
        "Set background document selection decoration and aspect ratio.",
        false,
    ),
    (
        "kxcapture_document_save",
        "Save a background document to an explicit absolute path.",
        false,
    ),
    (
        "kxcapture_document_close",
        "Close an owned background document.",
        false,
    ),
    (
        "kxcapture_document_recognize",
        "Start background recognition and return an owned job handle.",
        false,
    ),
    (
        "kxcapture_document_recognition_state",
        "Read retained recognition state at the expected document revision.",
        true,
    ),
    (
        "kxcapture_document_auto_filter",
        "Detect and filter selected categories in an owned background document through a cancelable job.",
        false,
    ),
    (
        "kxcapture_document_edit_recognition",
        if crate::edition::MINI {
            "Edit retained text recognition using its own revision."
        } else {
            "Edit retained text or table recognition using its own revision."
        },
        false,
    ),
    (
        "kxcapture_document_export_recognition",
        "Return, copy or save retained document recognition.",
        false,
    ),
    (
        "kxcapture_document_original_content",
        "Return, copy or save original source text or HTML.",
        false,
    ),
    (
        "kxcapture_job_list",
        "List background jobs owned by this client.",
        true,
    ),
    (
        "kxcapture_job_get",
        "Read an owned background job status and result.",
        true,
    ),
    (
        "kxcapture_job_cancel",
        "Request cancellation of an owned background job.",
        false,
    ),
    (
        "kxcapture_recording_state",
        "Read recording lifecycle state and revision.",
        true,
    ),
    (
        "kxcapture_recording_start",
        "Start a region recording with explicit recording options. post_processing enables rendering after Stop; post_processing_effect selects progress_bar or playback_time. progress_bar_color uses #RRGGBBAA (alpha last); #RRGGBB and color names are also accepted.",
        false,
    ),
    (
        "kxcapture_recording_control",
        "Pause, resume, finish, copy, or close an owned recording; cancel_render stops rendering, retry repeats it, keep_source preserves files, and discard explicitly removes them.",
        false,
    ),
    (
        "kxcapture_pinned_list",
        "List pinned images and their revisions.",
        true,
    ),
    (
        "kxcapture_pinned_get",
        "Read a pinned image state by stable ID.",
        true,
    ),
    (
        "kxcapture_pinned_create",
        "Pin a file, clipboard image, text, or HTML.",
        false,
    ),
    (
        "kxcapture_pinned_replace",
        "Replace a pinned image from a typed source.",
        false,
    ),
    (
        "kxcapture_pinned_update",
        "Update pinned image geometry and presentation properties.",
        false,
    ),
    (
        "kxcapture_pinned_action",
        "Show, hide, close, restore, or destroy pinned windows.",
        false,
    ),
    (
        "kxcapture_pinned_export",
        "Save or copy a pinned image.",
        false,
    ),
    (
        "kxcapture_pinned_edit",
        "Edit pinned annotations and recognized content.",
        false,
    ),
    ("kxcapture_group_list", "List pinned window groups.", true),
    (
        "kxcapture_group_update",
        "Create, activate, move to, or delete a pinned window group.",
        false,
    ),
];

pub fn schema(name: &str, input: Option<Value>) -> Result<Map<String, Value>, serde_json::Error> {
    if !crate::edition::method_enabled(name)
        || input
            .as_ref()
            .is_some_and(|value| !crate::edition::input_enabled(name, value))
    {
        return Err(serde::de::Error::custom(
            "This operation is unavailable in the compiled edition",
        ));
    }
    match name {
        "kxcapture_artifact_list" => model::<Empty>(input),
        "kxcapture_artifact_read" => model::<ArtifactRead>(input),
        "kxcapture_artifact_release" => model::<ArtifactRelease>(input),
        "kxcapture_app_status"
        | "kxcapture_app_displays"
        | "kxcapture_storage_status"
        | "kxcapture_permissions_get"
        | "kxcapture_updates_status"
        | "kxcapture_document_list"
        | "kxcapture_job_list"
        | "kxcapture_recording_state"
        | "kxcapture_group_list" => model::<Empty>(input),
        #[cfg(not(feature = "mini"))]
        "kxcapture_models_list" | "kxcapture_translation_catalog" => model::<Empty>(input),
        "kxcapture_app_action" => model::<AppActionInput>(input),
        "kxcapture_settings_get" => model::<Section>(input),
        "kxcapture_settings_update" => model::<Revision<SettingsUpdate>>(input),
        "kxcapture_settings_reset" => model::<Revision<SettingsReset>>(input),
        "kxcapture_settings_action" => model::<Revision<SettingsAction>>(input),
        #[cfg(not(feature = "mini"))]
        "kxcapture_models_update" => model::<Revision<ModelsUpdate>>(input),
        #[cfg(not(feature = "mini"))]
        "kxcapture_credentials_set" => model::<Revision<Credential>>(input),
        "kxcapture_history_list" => model::<HistoryList>(input),
        "kxcapture_history_get" => model::<HistoryGet>(input),
        "kxcapture_history_delete" => model::<Revision<HistoryId>>(input),
        "kxcapture_history_clear" => model::<Revision<Empty>>(input),
        "kxcapture_history_action" => model::<HistoryActionInput>(input),
        "kxcapture_configuration_export" => model::<FilePath>(input),
        "kxcapture_configuration_import" => model::<Revision<FilePath>>(input),
        "kxcapture_storage_cleanup" => model::<Revision<Cleanup>>(input),
        "kxcapture_permissions_request" => model::<Permission>(input),
        "kxcapture_updates_action" => model::<UpdateInput>(input),
        "kxcapture_templates_list" => model::<TemplateList>(input),
        "kxcapture_templates_update" => model::<Revision<TemplateUpdate>>(input),
        #[cfg(not(feature = "mini"))]
        "kxcapture_translation_start" => {
            let exclusive = [
                "texts",
                "source",
                "source_language",
                "target_language",
                "model_id",
            ];
            if input.as_ref().is_some_and(|v| {
                v.get("retry_job_id").is_some() && exclusive.iter().any(|key| v.get(key).is_some())
            }) {
                return Err(<serde_json::Error as serde::de::Error>::custom(
                    "retry_job_id cannot be combined with new translation input",
                ));
            }
            let listing = input.is_none();
            let mut schema = model::<TranslationStart>(input)?;
            if listing {
                schema.insert("allOf".into(), serde_json::json!([{"if":{"required":["retry_job_id"]},"then":{"not":{"anyOf":exclusive.iter().map(|key|serde_json::json!({"required":[key]})).collect::<Vec<_>>()}}}]));
            }
            Ok(schema)
        }
        "kxcapture_document_open" => model::<DocumentOpen>(input),
        "kxcapture_document_state" => model::<DocumentId>(input),
        "kxcapture_document_clone"
        | "kxcapture_document_close"
        | "kxcapture_document_undo"
        | "kxcapture_document_redo" => model::<DocumentMutation<Empty>>(input),
        "kxcapture_document_apply_annotations" => model::<DocumentMutation<Annotations>>(input),
        "kxcapture_document_set_tool" => model::<DocumentMutation<DocumentTool>>(input),
        "kxcapture_document_set_tool_style" => model::<DocumentToolStyleMutation>(input),
        "kxcapture_document_edit_elements" => {
            if input.as_ref().is_some_and(|v| {
                v.get("action").and_then(Value::as_str) == Some("erase_path")
                    && !v
                        .get("points")
                        .and_then(Value::as_array)
                        .is_some_and(|p| (1..=8192).contains(&p.len()))
            }) {
                return Err(serde::de::Error::custom(
                    "erase_path requires 1 to 8192 points",
                ));
            }
            model::<DocumentMutation<DocumentElementEdit>>(input)
        }
        "kxcapture_document_draw_template" => model::<DocumentMutation<DrawTemplate>>(input),
        "kxcapture_document_sample_color" => model::<DocumentMutation<DocumentPoint>>(input),
        "kxcapture_document_recapture" => model::<DocumentMutation<DocumentRecapture>>(input),
        "kxcapture_document_present" => model::<DocumentMutation<Empty>>(input),
        "kxcapture_document_set_selection" => model::<DocumentMutation<DocumentSelection>>(input),
        "kxcapture_document_render" | "kxcapture_document_copy" | "kxcapture_document_pin" => {
            model::<DocumentMutation<Render>>(input)
        }
        "kxcapture_document_set_selection_style" => {
            model::<DocumentMutation<SelectionStyle>>(input)
        }
        "kxcapture_document_save" => model::<DocumentMutation<DocumentSave>>(input),
        "kxcapture_document_recognize" => model::<DocumentMutation<DocumentRecognize>>(input),
        "kxcapture_document_auto_filter" => model::<DocumentMutation<PinnedAutoFilter>>(input),
        "kxcapture_document_recognition_state" => model::<DocumentMutation<Empty>>(input),
        "kxcapture_document_edit_recognition" => {
            model::<DocumentMutation<DocumentRecognitionEdit>>(input)
        }
        "kxcapture_document_export_recognition" => {
            model::<DocumentMutation<ExportRecognition>>(input)
        }
        "kxcapture_document_original_content" => {
            model::<DocumentMutation<OriginalContentOutput>>(input)
        }
        "kxcapture_job_get" | "kxcapture_job_cancel" => model::<JobId>(input),
        "kxcapture_recording_start" => model::<RecordingStart>(input),
        "kxcapture_recording_control" => model::<Revision<RecordingControl>>(input),
        "kxcapture_pinned_get" => model::<PinnedId>(input),
        "kxcapture_pinned_list" => model::<PinnedList>(input),
        "kxcapture_pinned_create" => model::<PinnedCreate>(input),
        "kxcapture_pinned_replace" => model::<Revision<PinnedReplace>>(input),
        "kxcapture_pinned_update" => model::<Revision<PinnedUpdate>>(input),
        "kxcapture_pinned_action" => {
            if input.as_ref().is_some_and(|v| {
                v.get("action").and_then(Value::as_str) != Some("restore_last")
                    && !v
                        .get("id")
                        .and_then(Value::as_str)
                        .is_some_and(|id| !id.is_empty())
            }) {
                return Err(serde::de::Error::custom("Pinned action requires an id"));
            }
            let generating = input.is_none();
            let mut schema = model::<Revision<PinnedActionInput>>(input)?;
            if generating {
                schema.insert("allOf".into(), serde_json::json!([{"if":{"properties":{"action":{"const":"restore_last"}},"required":["action"]},"then":{},"else":{"required":["id"]}}]));
            }
            Ok(schema)
        }
        "kxcapture_pinned_export" => model::<Revision<PinnedExport>>(input),
        "kxcapture_pinned_edit" => model::<Revision<PinnedEdit>>(input),
        "kxcapture_group_update" => model::<Revision<GroupUpdate>>(input),
        _ => Err(<serde_json::Error as serde::de::Error>::custom(
            "unknown tool",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn recording_schema_matches_application_validation() {
        fn primitive_contract(root: &Value, schema: &Value) -> Value {
            if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
                return primitive_contract(root, root.pointer(&reference[1..]).unwrap());
            }
            let mut result = Map::new();
            for key in ["type", "minimum", "maximum", "enum"] {
                if let Some(value) = schema.get(key) {
                    result.insert(key.into(), value.clone());
                }
            }
            if let Some(choices) = result.get_mut("enum").and_then(Value::as_array_mut) {
                choices.sort_unstable_by_key(Value::to_string);
            }
            Value::Object(result)
        }

        let catalog: Value =
            serde_json::from_str(include_str!("../../../mcp-capabilities.json")).unwrap();
        // Qt consumes this canonical contract through its generated header;
        // these Rust input types and serde validation remain independent.
        let expected = catalog["surface_contracts"]["recording_options"]["validation"]
            .as_object()
            .unwrap();
        let published = Value::Object(model::<RecordingOptions>(None).unwrap());
        let actual: Map<String, Value> = published["properties"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(name, schema)| (name.clone(), primitive_contract(&published, schema)))
            .collect();
        assert!(
            actual.keys().eq(expected.keys()),
            "Rust and Qt recording fields differ"
        );
        assert!(
            published.get("required").is_none(),
            "All recording overrides are optional"
        );
        assert!(model::<RecordingOptions>(Some(json!({}))).is_ok());

        for (name, contract) in expected {
            assert_eq!(
                actual[name],
                primitive_contract(&catalog, contract),
                "Rust recording field {name} differs from Qt validation"
            );
            let accepts = |value: Value| {
                model::<RecordingOptions>(Some(Value::Object(Map::from_iter([(
                    name.clone(),
                    value,
                )]))))
                .is_ok()
            };
            assert!(
                !accepts(Value::Null),
                "{name} must reject a supplied null override"
            );
            let wrong_type = match contract["type"].as_str().unwrap() {
                "integer" => json!(8.5),
                "boolean" => json!("true"),
                "string" => json!(true),
                kind => panic!("Unsupported application primitive: {kind}"),
            };
            assert!(
                !accepts(wrong_type),
                "{name} must reject the wrong input type"
            );
            if let Some(choices) = contract.get("enum").and_then(Value::as_array) {
                for choice in choices {
                    assert!(accepts(choice.clone()), "{name} must accept {choice}");
                }
                let invalid = if contract["type"] == "integer" {
                    json!(0)
                } else {
                    json!("unsupported_recording_option")
                };
                assert!(!accepts(invalid), "{name} must reject an unknown choice");
            } else if contract["type"] == "integer" {
                let minimum = contract["minimum"].as_i64().unwrap();
                let maximum = contract["maximum"].as_i64().unwrap();
                assert!(accepts(json!(minimum)), "{name} must accept its minimum");
                assert!(accepts(json!(maximum)), "{name} must accept its maximum");
                assert!(
                    !accepts(json!(minimum - 1)),
                    "{name} must enforce its minimum"
                );
                assert!(
                    !accepts(json!(maximum + 1)),
                    "{name} must enforce its maximum"
                );
            }
        }
    }
    #[test]
    fn recording_mouse_highlight_radius_matches_application_bounds() {
        for radius in [8, 24, 128] {
            assert!(
                schema(
                    "kxcapture_recording_start",
                    Some(json!({
                        "region": [0, 0, 320, 240],
                        "options": {"mouse_highlight": true, "mouse_highlight_radius": radius}
                    }))
                )
                .is_ok(),
                "supported radius {radius}"
            );
        }
        for radius in [
            json!(-1),
            json!(7),
            json!(129),
            json!(8.5),
            json!("24"),
            json!(true),
        ] {
            assert!(
                schema(
                    "kxcapture_recording_start",
                    Some(json!({
                        "region": [0, 0, 320, 240],
                        "options": {"mouse_highlight_radius": radius}
                    }))
                )
                .is_err(),
                "invalid radius {radius}"
            );
        }
        let published = schema("kxcapture_recording_start", None).unwrap();
        let radius = &published["$defs"]["MouseHighlightRadius"];
        assert_eq!(radius["type"], "integer");
        assert_eq!(radius["minimum"], 8);
        assert_eq!(radius["maximum"], 128);
    }
    #[test]
    fn recording_capability_catalogs_match_typed_options_without_duplicates() {
        let published = model::<RecordingOptions>(None).unwrap();
        let properties = published["properties"].as_object().unwrap();
        let expected: std::collections::BTreeSet<_> =
            properties.keys().map(String::as_str).collect();
        for (edition, source) in [
            ("full", include_str!("../../../mcp-capabilities.json")),
            ("mini", include_str!("../../../mcp-capabilities-mini.json")),
        ] {
            let catalog: Value = serde_json::from_str(source).unwrap();
            let members = catalog["surface_contracts"]["recording_options"]["members"]
                .as_array()
                .unwrap();
            let actual: std::collections::BTreeSet<_> = members
                .iter()
                .map(|member| member.as_str().unwrap())
                .collect();
            assert_eq!(actual.len(), members.len(), "{edition}: duplicate options");
            assert_eq!(
                actual, expected,
                "{edition}: recording options differ from schema"
            );
        }
    }
    #[test]
    fn recording_post_processing_options_and_retained_source_actions_are_typed() {
        for effect in ["progress_bar", "playback_time"] {
            assert!(
                schema(
                    "kxcapture_recording_start",
                    Some(json!({
                        "region": [0, 0, 320, 240],
                        "options": {"post_processing": true, "post_processing_effect": effect,
                                    "progress_bar_color": "#1464C880", "quality": 70,
                                    "separate_audio_tracks": true}
                    }))
                )
                .is_ok()
            );
        }
        for (key, value) in [
            ("post_processing", json!(1)),
            ("post_processing_effect", json!("both")),
            ("progress_bar_color", json!(42)),
            ("render_mode", json!("post_recording")),
            ("quality", json!(101)),
        ] {
            let mut input = json!({"region": [0,0,320,240], "options": {}});
            input["options"][key] = value;
            assert!(
                schema("kxcapture_recording_start", Some(input)).is_err(),
                "{key}"
            );
        }
        for action in ["cancel", "cancel_render", "retry", "keep_source", "discard"] {
            assert!(
                schema(
                    "kxcapture_recording_control",
                    Some(json!({
                        "recording_id": "recording", "expected_revision": 1, "action": action
                    }))
                )
                .is_ok(),
                "{action}"
            );
        }
    }
    #[test]
    fn recording_accepts_only_boolean_separate_audio_tracks() {
        for value in [serde_json::json!(true), serde_json::json!(false)] {
            assert!(
                schema(
                    "kxcapture_recording_start",
                    Some(serde_json::json!({
                        "region": [0, 0, 320, 240], "options": {"separate_audio_tracks": value}
                    }))
                )
                .is_ok()
            );
        }
        assert!(
            schema(
                "kxcapture_recording_start",
                Some(serde_json::json!({
                    "region": [0, 0, 320, 240], "options": {"separate_audio_tracks": 1}
                }))
            )
            .is_err()
        );
    }

    #[test]
    fn recording_accepts_only_boolean_record_mouse_scrolling() {
        for value in [serde_json::json!(true), serde_json::json!(false)] {
            assert!(
                schema(
                    "kxcapture_recording_start",
                    Some(serde_json::json!({
                        "region": [0, 0, 320, 240], "options": {"record_mouse_scrolling": value}
                    }))
                )
                .is_ok()
            );
        }
        assert!(
            schema(
                "kxcapture_recording_start",
                Some(serde_json::json!({
                    "region": [0, 0, 320, 240], "options": {"record_mouse_scrolling": 1}
                }))
            )
            .is_err()
        );
    }

    #[test]
    fn all_domain_contracts_have_unique_schemas_and_mutations_are_revisioned() {
        let mut names = std::collections::HashSet::new();
        for (name, _, _) in TOOLS
            .iter()
            .filter(|(name, _, _)| crate::edition::method_enabled(name))
        {
            assert!(names.insert(name), "{name}");
            assert!(
                schema(name, None).unwrap().get("type") == Some(&json!("object")),
                "{name}"
            );
        }
        assert!(schema("kxcapture_settings_update", Some(json!({"values":{}}))).is_err());
        assert!(schema("kxcapture_document_apply_annotations",Some(json!({"document_id":"d","expected_revision":1,"operations":[{"type":"execute"}]}))).is_err());
        assert!(schema("kxcapture_document_apply_annotations",Some(json!({"document_id":"d","expected_revision":1,"operations":[{"type":"rectangle","bounds":[0,0,20,20],"style":{}}]}))).is_ok());
        assert!(
            schema(
                "kxcapture_settings_update",
                Some(json!({"expected_revision":2,"values":{"general.language":"en_US"}}))
            )
            .is_ok()
        );
        assert!(
            schema(
                "kxcapture_document_open",
                Some(json!({"source":"capture","target":"focused_window","delay_seconds":10}))
            )
            .is_ok()
        );
        assert!(
            schema(
                "kxcapture_document_open",
                Some(json!({"source":"capture","delay_seconds":11}))
            )
            .is_err()
        );
    }
    #[test]
    fn recording_limits_and_nested_pin_operations_are_validated() {
        for action in [
            "show",
            "hide",
            "close",
            "destroy",
            "show_all",
            "hide_others",
            "close_others",
            "close_all",
        ] {
            assert!(
                schema(
                    "kxcapture_pinned_action",
                    Some(json!({"id":"pin","expected_revision":1,"action":action}))
                )
                .is_ok(),
                "{action}"
            );
            assert!(
                schema(
                    "kxcapture_pinned_action",
                    Some(json!({"expected_revision":1,"action":action}))
                )
                .is_err(),
                "{action}"
            );
        }
        assert!(
            schema(
                "kxcapture_pinned_action",
                Some(json!({"expected_revision":1,"action":"restore_last"}))
            )
            .is_ok()
        );
        let input = json!({"region":[0,0,1920,1080],"options":{"frame_rate":60,"clarity":"1080p","keyboard_size":32,"quality":100,"path":"C:/capture.mp4"}});
        assert!(schema("kxcapture_recording_start", Some(input.clone())).is_ok());
        for (key, value) in [
            ("frame_rate", json!(240)),
            ("quality", json!(101)),
            ("quality", json!(-1)),
            ("quality", json!(1.5)),
            ("clarity", json!("8k")),
            ("keyboard_size", json!(256)),
            ("surprise", json!(true)),
        ] {
            let mut invalid = input.clone();
            invalid["options"][key] = value;
            assert!(
                schema("kxcapture_recording_start", Some(invalid)).is_err(),
                "{key}"
            );
        }
        assert!(
            schema(
                "kxcapture_pinned_create",
                Some(json!({"source":{"kind":"file","path":"C:/image.png","command":"bad"}}))
            )
            .is_err()
        );
        assert!(schema("kxcapture_pinned_edit",Some(json!({"id":"p","expected_revision":1,"action":"recognize","payload":{"kind":"text"}}))).is_ok());
    }

    #[test]
    fn pinned_shadow_visibility_is_an_independent_boolean_property() {
        for border in [false, true] {
            for shadow in [false, true] {
                assert!(
                    schema(
                        "kxcapture_pinned_update",
                        Some(json!({"id":"pin", "expected_revision":1,
                            "properties":{"show_border":border,"show_shadow":shadow}}))
                    )
                    .is_ok()
                );
            }
        }
        for invalid in [json!(1), json!("true")] {
            assert!(
                schema(
                    "kxcapture_pinned_update",
                    Some(json!({"id":"pin", "expected_revision":1,
                        "properties":{"show_shadow":invalid}}))
                )
                .is_err()
            );
        }
    }
    #[test]
    fn document_capture_recognition_and_artifact_contracts_have_distinct_bounds() {
        assert!(
            schema(
                "kxcapture_document_save",
                Some(json!({"document_id":"d","expected_revision":1,"automatic_path":true}))
            )
            .is_err()
        );
        assert!(schema("kxcapture_document_save", Some(json!({"document_id":"d","expected_revision":1,"path":"C:/output.pdf","format":"pdf","quality":90,"compression_level":"medium","pdf_page_size":"a4_landscape","pdf_title":"Fixture"}))).is_ok());
        assert!(schema("kxcapture_document_edit_elements", Some(json!({"document_id":"d","expected_revision":1,"action":"erase_path","points":[[1,2],[3,4]]}))).is_ok());
        assert!(schema("kxcapture_document_edit_elements", Some(json!({"document_id":"d","expected_revision":1,"action":"erase_path","points":[]}))).is_err());
        assert!(super::super::schema("kxcapture_screenshot_edit_elements", Some(json!({"session_id":"s","expected_revision":1,"action":"erase_path","points":[[1,2]]}))).is_err());
        assert!(schema("kxcapture_pinned_edit", Some(json!({"id":"p","expected_revision":1,"action":"tool_style","payload":{"target":"arrow","style":{"arrow_ratio":2}}}))).is_ok());
        assert!(
            schema(
                "kxcapture_document_open",
                Some(
                    json!({"source":"capture","target":"monitor","monitor_id":"primary","scale":4})
                )
            )
            .is_ok()
        );
        assert!(
            schema(
                "kxcapture_document_open",
                Some(json!({"source":"capture","scale":4.01}))
            )
            .is_err()
        );
        assert!(schema("kxcapture_document_recapture", Some(json!({"document_id":"one","expected_revision":1,"idempotency_key":"repeat","source":"file","path":"C:/source.png"}))).is_ok());
        assert!(
            schema(
                "kxcapture_document_set_tool",
                Some(json!({"document_id":"one","expected_revision":1,"tool":"ocr"}))
            )
            .is_err()
        );
        assert!(schema("kxcapture_document_edit_recognition", Some(json!({"document_id":"one","expected_revision":1,"expected_recognition_revision":2,"action":"set_text","text":"edited"}))).is_ok());
        assert!(schema("kxcapture_document_edit_recognition", Some(json!({"document_id":"one","expected_revision":1,"action":"set_text","text":"edited"}))).is_err());
        for bytes in [0, 262145] {
            assert!(
                schema(
                    "kxcapture_artifact_read",
                    Some(json!({"artifact_id":"a","max_bytes":bytes}))
                )
                .is_err()
            );
        }
        assert!(
            schema(
                "kxcapture_artifact_read",
                Some(json!({"artifact_id":"a","offset":262144,"max_bytes":262144}))
            )
            .is_ok()
        );
        assert_eq!(
            schema(
                "kxcapture_translation_start",
                Some(json!({"source":"selection"}))
            )
            .is_ok(),
            !crate::edition::MINI && cfg!(feature = "translation")
        );
        assert_eq!(
            schema(
                "kxcapture_translation_start",
                Some(json!({"retry_job_id":"prior"}))
            )
            .is_ok(),
            !crate::edition::MINI && cfg!(feature = "translation")
        );
        assert!(
            schema(
                "kxcapture_translation_start",
                Some(json!({"retry_job_id":"prior","texts":["new"]}))
            )
            .is_err()
        );
    }
}

#[cfg(test)]
mod audio_gain_tests {
    use super::*;
    #[test]
    fn recording_audio_gains_validate_signed_integer_bounds() {
        for value in [-24, 0, 24] {
            let options: RecordingOptions = serde_json::from_value(
                serde_json::json!({"microphone_gain_db":value,"system_audio_gain_db":-value}),
            )
            .unwrap();
            assert_eq!(options.microphone_gain_db.0.unwrap().0, value);
            assert_eq!(options.system_audio_gain_db.0.unwrap().0, -value);
        }
        for value in [
            serde_json::json!(-25),
            serde_json::json!(25),
            serde_json::json!(0.5),
            serde_json::json!("0"),
            serde_json::json!(true),
        ] {
            assert!(
                serde_json::from_value::<RecordingOptions>(
                    serde_json::json!({"system_audio_gain_db":value})
                )
                .is_err()
            );
        }
    }
}

#[cfg(test)]
mod keyboard_filter_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn recording_keyboard_filters_accept_only_supplied_boolean_values() {
        let omitted: RecordingOptions = serde_json::from_value(json!({})).unwrap();
        assert!(omitted.record_number_keys.0.is_none());
        assert!(omitted.record_letters_and_symbols.0.is_none());
        assert!(omitted.record_control_keys.0.is_none());
        assert!(omitted.preserve_keyboard_shortcuts.0.is_none());
        let published = model::<RecordingOptions>(None).unwrap();
        for key in [
            "record_number_keys",
            "record_letters_and_symbols",
            "record_control_keys",
            "preserve_keyboard_shortcuts",
        ] {
            assert_eq!(published["properties"][key]["type"], "boolean", "{key}");
            for value in [json!(true), json!(false)] {
                let options = Value::Object(Map::from_iter([(key.into(), value)]));
                assert!(serde_json::from_value::<RecordingOptions>(options.clone()).is_ok());
                assert!(
                    schema(
                        "kxcapture_recording_start",
                        Some(json!({"region": [0, 0, 320, 240], "options": options}))
                    )
                    .is_ok(),
                    "{key}"
                );
            }
            for value in [json!(1), json!("true"), Value::Null, json!([]), json!({})] {
                let options = Value::Object(Map::from_iter([(key.into(), value)]));
                assert!(serde_json::from_value::<RecordingOptions>(options.clone()).is_err());
                assert!(
                    schema(
                        "kxcapture_recording_start",
                        Some(json!({"region": [0, 0, 320, 240], "options": options}))
                    )
                    .is_err(),
                    "{key}"
                );
            }
        }
    }
}

#[cfg(test)]
mod webcam_tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn recording_webcam_options_validate_types_and_style_boundaries() {
        for size in [50, 150, 500] {
            for shape in ["square", "circle"] {
                assert!(
                    schema(
                        "kxcapture_recording_start",
                        Some(json!({
                            "region": [0, 0, 320, 240],
                            "options": {"webcam": true, "webcam_size": size, "webcam_shape": shape}
                        }))
                    )
                    .is_ok()
                );
            }
        }
        for (key, value) in [
            ("webcam", json!(1)),
            ("webcam_size", json!(49)),
            ("webcam_size", json!(501)),
            ("webcam_size", json!(150.5)),
            ("webcam_shape", json!("oval")),
        ] {
            let mut input = json!({"region": [0, 0, 320, 240], "options": {}});
            input["options"][key] = value;
            assert!(
                schema("kxcapture_recording_start", Some(input)).is_err(),
                "{key}"
            );
        }
    }
}
