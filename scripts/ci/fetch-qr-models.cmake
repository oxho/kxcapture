# The macOS install script stores absolute paths to artifacts/qr-models-wechat
# from the build job. Recreate that directory before cmake --install on a
# different runner. Hashes live in cmake/FetchSnowShotQrModels.cmake.
include("${CMAKE_CURRENT_LIST_DIR}/../../cmake/FetchSnowShotQrModels.cmake")
snow_shot_fetch_qr_models(_kxcapture_qr_models)
message(STATUS "WeChat QR models are in ${_kxcapture_qr_models}")
