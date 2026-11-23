include_guard(GLOBAL)
find_package(Python3 REQUIRED COMPONENTS Interpreter)
set(_snow_sparkle_repo "${CMAKE_CURRENT_LIST_DIR}/..")
set(SNOW_SPARKLE_TOOL "${_snow_sparkle_repo}/scripts/snow-shot-sparkle.py")
set(SNOW_SPARKLE_MANIFEST "${_snow_sparkle_repo}/snow_shot/packaging/macos/sparkle.json")
file(READ "${SNOW_SPARKLE_MANIFEST}" _snow_sparkle_manifest)
string(JSON SNOW_SPARKLE_VERSION GET "${_snow_sparkle_manifest}" version)
string(JSON SNOW_SPARKLE_PUBLIC_KEY GET "${_snow_sparkle_manifest}" publicKey)
set(SNOW_SPARKLE_ROOT "${_snow_sparkle_repo}/.tools/sparkle/${SNOW_SPARKLE_VERSION}" CACHE PATH "Pinned Sparkle distribution")
set(SNOW_SPARKLE_FRAMEWORK "${SNOW_SPARKLE_ROOT}/Sparkle.xcframework/macos-arm64_x86_64/Sparkle.framework")
if(NOT EXISTS "${SNOW_SPARKLE_FRAMEWORK}/Sparkle")
    message(FATAL_ERROR "Run python3 scripts/snow-shot-sparkle.py bootstrap before configuring macOS")
endif()
if(SNOW_APPS_PACKAGE_SNOW_SHOT AND SNOW_SPARKLE_PUBLIC_KEY STREQUAL "")
    message(WARNING
        "Sparkle public key is unset. Automatic updates stay disabled until a KXCapture key is configured.")
endif()
set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS "${SNOW_SPARKLE_MANIFEST}")
set(SNOW_SPARKLE_FEED_ARCH arm64)
if(CMAKE_OSX_ARCHITECTURES STREQUAL "x86_64")
    set(SNOW_SPARKLE_FEED_ARCH x86_64)
endif()
# Release builds keep these empty. A local debug feed, not a production URL, is
# http://127.0.0.1:8782/sparkle/ (appcasts kxcapture-macos-arm64.xml and
# kxcapture-macos-x86_64.xml). Sparkle will not accept that http feed unless a
# debug-only path sets allowLocalHttp; release and store packages must leave it off.
set(SNOW_SPARKLE_GITHUB_FEED_BASE "" CACHE STRING
    "Primary Sparkle feed directory. Empty disables the feed. Local debug only: http://127.0.0.1:8782/sparkle")
set(SNOW_SPARKLE_GITEE_FEED_BASE "" CACHE STRING
    "Mirror Sparkle feed directory. Empty disables the mirror.")
foreach(_base IN ITEMS SNOW_SPARKLE_GITHUB_FEED_BASE SNOW_SPARKLE_GITEE_FEED_BASE)
    if(NOT "${${_base}}" STREQUAL "" AND
       NOT "${${_base}}" MATCHES "^https://[A-Za-z0-9][A-Za-z0-9./_-]*$")
        message(FATAL_ERROR "${_base} must be empty or an HTTPS URL")
    endif()
endforeach()
if(SNOW_SPARKLE_GITHUB_FEED_BASE STREQUAL "")
    set(SNOW_SPARKLE_APPCAST_URL "")
else()
    set(SNOW_SPARKLE_APPCAST_URL
        "${SNOW_SPARKLE_GITHUB_FEED_BASE}/kxcapture-macos-${SNOW_SPARKLE_FEED_ARCH}.xml")
endif()
if(SNOW_SPARKLE_GITEE_FEED_BASE STREQUAL "")
    set(SNOW_SPARKLE_MIRROR_APPCAST_URL "")
else()
    set(SNOW_SPARKLE_MIRROR_APPCAST_URL
        "${SNOW_SPARKLE_GITEE_FEED_BASE}/kxcapture-macos-${SNOW_SPARKLE_FEED_ARCH}.xml")
endif()
if(SNOW_SPARKLE_GITHUB_FEED_BASE STREQUAL "")
    set(SNOW_SPARKLE_MINI_APPCAST_URL "")
else()
    set(SNOW_SPARKLE_MINI_APPCAST_URL
        "${SNOW_SPARKLE_GITHUB_FEED_BASE}/kxcapture-mini-macos-${SNOW_SPARKLE_FEED_ARCH}.xml")
endif()
if(SNOW_SPARKLE_GITEE_FEED_BASE STREQUAL "")
    set(SNOW_SPARKLE_MINI_MIRROR_APPCAST_URL "")
else()
    set(SNOW_SPARKLE_MINI_MIRROR_APPCAST_URL
        "${SNOW_SPARKLE_GITEE_FEED_BASE}/kxcapture-mini-macos-${SNOW_SPARKLE_FEED_ARCH}.xml")
endif()
add_library(SnowSparkle SHARED IMPORTED GLOBAL)
set_target_properties(SnowSparkle PROPERTIES
    IMPORTED_LOCATION "${SNOW_SPARKLE_FRAMEWORK}/Sparkle"
    FRAMEWORK TRUE)
function(snow_embed_sparkle target bundle component)
    add_custom_command(TARGET ${target} POST_BUILD
        COMMAND "${CMAKE_COMMAND}" -E make_directory "$<TARGET_BUNDLE_DIR:${target}>/Contents/Frameworks"
        COMMAND /usr/bin/ditto "${SNOW_SPARKLE_FRAMEWORK}"
            "$<TARGET_BUNDLE_DIR:${target}>/Contents/Frameworks/Sparkle.framework"
        VERBATIM)
    set_property(TARGET ${target} APPEND PROPERTY BUILD_RPATH "@executable_path/../Frameworks")
    install(DIRECTORY "${SNOW_SPARKLE_FRAMEWORK}"
        DESTINATION "${bundle}/Contents/Frameworks" USE_SOURCE_PERMISSIONS COMPONENT ${component})
    set(_product kxcapture)
    if(component STREQUAL "SnowShotMini")
        set(_product kxcapture-mini)
    endif()
    install(FILES "${SNOW_SPARKLE_ROOT}/LICENSE"
        DESTINATION "${bundle}/Contents/Resources/${_product}/licenses/third-party/sparkle"
        COMPONENT ${component})
endfunction()
