#include "snow_shot/app/mcp/mcpedition.h"
#include "snow_shot/presentation/screenshotrecognitionfileexport.h"

#include <QCoreApplication>
#include <QJsonObject>
#include <cstdlib>
#include <iostream>
#include <type_traits>

namespace {
template <class T, class = void> struct IsComplete : std::false_type {};
template <class T> struct IsComplete<T, std::void_t<decltype(sizeof(T))>> : std::true_type {};

static_assert(IsComplete<ScreenshotRecognitionFileExport>::value ==
              (snow_shot::app::edition::imageConversion ||
               snow_shot::app::edition::latexRecognition ||
               snow_shot::app::edition::qrRecognition));

void require(bool condition, const char* message) {
    if (!condition) {
        std::cerr << message << '\n';
        std::exit(EXIT_FAILURE);
    }
}
} // namespace

int main(int argc, char** argv) {
    QCoreApplication app(argc, argv);
    using namespace snow_shot::app;
    using namespace snow_shot::app::mcp;
    for (const auto* method :
         {"kxcapture_models_list", "kxcapture_models_update", "kxcapture_credentials_set"})
        require(editionMethodEnabled(QString::fromLatin1(method)) == edition::apiConfiguration,
                "API configuration tools must match the compiled edition");
    for (const auto* method : {"kxcapture_screenshot_translate", "kxcapture_translation_catalog",
                               "kxcapture_translation_start"})
        require(editionMethodEnabled(QString::fromLatin1(method)) == edition::textTranslation,
                "Translation tools must match the compiled edition");
    for (const auto* method : {"kxcapture_document_recognize", "kxcapture_screenshot_recognize"}) {
        const auto name = QString::fromLatin1(method);
        require(editionRequestEnabled(name, {{QStringLiteral("kind"), QStringLiteral("text")}}),
                "Manual text recognition must remain available");
        for (const auto* kind : {"qr", "table", "latex", "markdown", "html", "document"}) {
            const auto value = QString::fromLatin1(kind);
            require(editionRequestEnabled(name, {{QStringLiteral("kind"), value}}) ==
                        editionRecognitionKindEnabled(value),
                    "Recognition requests must respect feature flags");
        }
    }
    require(
        editionRequestEnabled(QStringLiteral("kxcapture_pinned_edit"),
                              {{QStringLiteral("action"), QStringLiteral("recognize")},
                               {QStringLiteral("payload"),
                                QJsonObject{{QStringLiteral("kind"), QStringLiteral("text")}}}}),
        "Pinned manual OCR must remain available");
    require(editionRequestEnabled(QStringLiteral("kxcapture_pinned_edit"),
                                  {{QStringLiteral("action"), QStringLiteral("translate")}}) ==
                edition::textTranslation,
            "Pinned translation must respect feature flags");
    require(editionRequestEnabled(QStringLiteral("kxcapture_document_edit_recognition"),
                                  {{QStringLiteral("action"), QStringLiteral("set_cell")}}) ==
                edition::tableRecognition,
            "Table editing cannot bypass disabled recognition");
    require(editionRequestEnabled(QStringLiteral("kxcapture_screenshot_export_recognition"),
                                  {{QStringLiteral("format"), QStringLiteral("markdown")}}) ==
                edition::imageConversion,
            "Image conversion cannot bypass disabled recognition");
    require(editionRequestEnabled(QStringLiteral("kxcapture_document_save"), {}),
            "Unaffected document export must remain available");
    return EXIT_SUCCESS;
}
