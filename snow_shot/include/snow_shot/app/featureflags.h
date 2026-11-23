#ifndef SNOW_SHOT_APP_FEATUREFLAGS_H
#define SNOW_SHOT_APP_FEATUREFLAGS_H

#include "snow_shot/presentation/globalshortcuttypes.h"

#include <QString>

namespace snow_shot::features {
// Defaults stay on so unit tests, which never enter the application main, keep
// the historical surface. The KXCapture executable applies the CMake options
// before any window, shortcut, or settings catalog is created.
inline bool& translationEnabledFlag() {
    static bool enabled = true;
    return enabled;
}
inline bool& ocrEnabledFlag() {
    static bool enabled = true;
    return enabled;
}
[[nodiscard]] inline bool translationEnabled() {
    return translationEnabledFlag();
}
inline void setTranslationEnabled(bool enabled) {
    translationEnabledFlag() = enabled;
}
[[nodiscard]] inline bool ocrEnabled() {
    return ocrEnabledFlag();
}
inline void setOcrEnabled(bool enabled) {
    ocrEnabledFlag() = enabled;
}

[[nodiscard]] inline bool shortcutActionEnabled(presentation::GlobalShortcutAction action) {
    using Action = presentation::GlobalShortcutAction;
    if (action == Action::ScreenshotTranslation || action == Action::TranslateSelectedText)
        return translationEnabled();
    if (action == Action::ScreenshotOcr || action == Action::ScreenshotOcrCopy)
        return ocrEnabled();
    return true;
}

[[nodiscard]] inline bool configurationKeyEnabled(const QString& key) {
    if (key.isEmpty())
        return true;
    if (!translationEnabled() &&
        (key.startsWith(QStringLiteral("screenshot_translation/")) ||
         key.startsWith(QStringLiteral("extended_features/translation")) ||
         key.startsWith(QStringLiteral("extended_features/jump_to_translation")) ||
         key == QStringLiteral("interface/translation_window_size") ||
         key.endsWith(QStringLiteral("/screenshot_translation")) ||
         key.endsWith(QStringLiteral("/translate_selected_text")) ||
         key.endsWith(QStringLiteral("/text_translation"))))
        return false;
    if (!ocrEnabled() &&
        (key.startsWith(QStringLiteral("text_recognition/")) ||
         key.endsWith(QStringLiteral("/screenshot_ocr")) ||
         key.endsWith(QStringLiteral("/screenshot_ocr_copy")) ||
         key == QStringLiteral("screenshot/auto_execute_after_text_recognition") ||
         key == QStringLiteral("pin_to_screen/automatic_text_recognition") ||
         key.endsWith(QStringLiteral("/text_recognition"))))
        return false;
    return true;
}

[[nodiscard]] inline bool settingsItemIdEnabled(const QString& id) {
    if (!translationEnabled() && id != QStringLiteral("text-recognition-translation") &&
        (id == QStringLiteral("translation") || id.startsWith(QStringLiteral("translation.")) ||
         id.contains(QStringLiteral("translate")) ||
         id.contains(QStringLiteral("text-translation")) ||
         id.contains(QStringLiteral("screenshot-translation"))))
        return false;
    if (!ocrEnabled() &&
        (id.contains(QStringLiteral("text-recognition")) ||
         id.contains(QStringLiteral("screenshot-ocr")) || id.startsWith(QStringLiteral("ocr")) ||
         id.contains(QStringLiteral(".ocr"))))
        return false;
    return true;
}
} // namespace snow_shot::features

#endif
