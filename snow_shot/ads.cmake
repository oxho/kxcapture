# KXCapture 自有广告模块。可被主工程 include() 或 add_subdirectory()。
# 拷进仓库后的位置：本文件建议放在 snow_shot/ads.cmake 或 snow_shot/CMakeLists 里 include；
# 头文件 snow_shot/include/snow_shot/ads/，源码 snow_shot/src/ads/。
# 库目标名 snow_shot_ads（下划线）；测试可执行文件 snow-shot-ads-tests（连字符）。
#
# 主工程必须先通过 cmake/SnowQt.cmake 的 snow_find_qt 解析 Qt 6.12.0。
# 本文件在被主工程引入时不调用 find_package(Qt6)，尤其不写不带版本的 find_package。

if(CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_SOURCE_DIR)
    # 仅单独在本目录自测时走这条路径（box 上验证用）。仓库构建不要走这里。
    cmake_minimum_required(VERSION 3.21)
    project(snow_shot_ads_standalone LANGUAGES CXX)
    find_package(Qt6 6.8.2 REQUIRED COMPONENTS Core Gui Widgets Network Test)
    set(SNOW_SHOT_ADS_BUILD_TESTS ON CACHE BOOL "Build snow-shot-ads-tests" FORCE)
endif()

if(NOT TARGET Qt6::Widgets OR NOT TARGET Qt6::Network)
    message(FATAL_ERROR
        "snow_shot_ads: 请先在主工程里 snow_find_qt(COMPONENTS Widgets Network)，再 include 本文件")
endif()

add_library(snow_shot_ads STATIC
    ${CMAKE_CURRENT_LIST_DIR}/src/ads/adservice.cpp
    ${CMAKE_CURRENT_LIST_DIR}/src/ads/adbannerwidget.cpp
    ${CMAKE_CURRENT_LIST_DIR}/src/ads/adcarouselwidget.cpp
    ${CMAKE_CURRENT_LIST_DIR}/include/snow_shot/ads/adtypes.h
    ${CMAKE_CURRENT_LIST_DIR}/include/snow_shot/ads/adservice.h
    ${CMAKE_CURRENT_LIST_DIR}/include/snow_shot/ads/adbannerwidget.h
    ${CMAKE_CURRENT_LIST_DIR}/include/snow_shot/ads/adcarouselwidget.h)
target_include_directories(snow_shot_ads PUBLIC ${CMAKE_CURRENT_LIST_DIR}/include)
target_compile_features(snow_shot_ads PUBLIC cxx_std_17)
set_target_properties(snow_shot_ads PROPERTIES AUTOMOC ON POSITION_INDEPENDENT_CODE ON)
target_link_libraries(snow_shot_ads PUBLIC Qt6::Widgets Qt6::Network)

option(SNOW_SHOT_ADS_BUILD_TESTS "Build snow-shot-ads-tests" OFF)
if(SNOW_SHOT_ADS_BUILD_TESTS)
    if(NOT TARGET Qt6::Test)
        message(FATAL_ERROR "snow-shot-ads-tests 需要主工程先 snow_find_qt(COMPONENTS Test)，或单独构建本目录")
    endif()
    add_executable(snow-shot-ads-tests ${CMAKE_CURRENT_LIST_DIR}/tests/ads_tests.cpp)
    set_target_properties(snow-shot-ads-tests PROPERTIES AUTOMOC ON)
    target_link_libraries(snow-shot-ads-tests PRIVATE snow_shot_ads Qt6::Test)
endif()
