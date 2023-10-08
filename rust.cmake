set(ENV{CARGO_TARGET_DIR})
message(STATUS $ENV{CARGO_TARGET_DIR})

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_BUILD_TYPE "")
  set(TARGET_DIR "debug")
else()
  set(CARGO_BUILD_TYPE "--release")
  set(TARGET_DIR "release")
endif()

if("$ENV{BUILD_TYPE}" STREQUAL "Debug")
  set(CARGO_BUILD_TYPE "")
  set(TARGET_DIR "debug")
endif()

if(CMAKE_HOST_SYSTEM_NAME MATCHES "Linux")
  set(NATIVE_LIBRARY "../target/${TARGET_DIR}/libnative.so")
elseif(CMAKE_HOST_SYSTEM_NAME MATCHES "Windows")
  set(NATIVE_LIBRARY "../target/${TARGET_DIR}/native.dll")
else()
  set(NATIVE_LIBRARY "non-exist")
endif()

set(CARGO_MANIFEST_PATH "--manifest-path=../native/Cargo.toml")

message(CMAKE_BUILD_TYPE " = " ${CMAKE_BUILD_TYPE})
message(BUILD_TYPE " = " $ENV{BUILD_TYPE})
message(NATIVE_LIBRARY " = " ${NATIVE_LIBRARY})
message(TARGET_DIR " = " ${TARGET_DIR})

add_custom_target(native ALL
  COMMAND cargo build ${CARGO_BUILD_TYPE} ${CARGO_MANIFEST_PATH}
  # COMMAND ${CMAKE_COMMAND} -E copy ${NATIVE_LIBRARY} ${INSTALL_BUNDLE_LIB_DIR}/
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
)

install(FILES "${NATIVE_LIBRARY}" DESTINATION "${INSTALL_BUNDLE_LIB_DIR}"
  COMPONENT Runtime)
