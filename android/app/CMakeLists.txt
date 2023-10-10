cmake_minimum_required(VERSION 3.22.1)
project("native")

set(ANDROID_PLATFORM android-21)
set(ENV{CARGO_TARGET_DIR})

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_BUILD_TYPE "")
else()
  set(CARGO_BUILD_TYPE "--release")
endif()

message(CMAKE_BUILD_TYPE " = " ${CMAKE_BUILD_TYPE})
message(ANDROID_PLATFORM " = " ${ANDROID_PLATFORM})
message(ANDROID_ABI " = " ${ANDROID_ABI})
message(ANDROID_NDK " = " ${ANDROID_NDK})
message(CARGO_BUILD_TYPE " = " ${CARGO_BUILD_TYPE})
message(BUILD_TYPE " = " $ENV{BUILD_TYPE})

add_custom_target(${PROJECT_NAME} ALL
  COMMAND cargo ndk
    -t ${ANDROID_ABI}
    -o src/main/jniLibs
    --manifest-path ../../${PROJECT_NAME}/Cargo.toml
    build ${CARGO_BUILD_TYPE}
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)
