# We include Corrosion inline here, but ideally in a project with
# many dependencies we would need to install Corrosion on the system.
# See instructions on https://github.com/corrosion-rs/corrosion#cmake-install
# Once done, uncomment this line:
set(Corrosion_DIR ${CMAKE_CURRENT_SOURCE_DIR}/cmake)
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake)
find_package(Corrosion REQUIRED)

# include(FetchContent)

# FetchContent_Declare(
#     Corrosion
#     GIT_REPOSITORY git@github.com:corrosion-rs/corrosion.git
#     GIT_TAG v0.4.3 # Optionally specify a version tag or branch here
# )

# FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ../ffi/Cargo.toml IMPORTED_CRATES imported_crates)

target_link_libraries(${BINARY_NAME} PRIVATE ${imported_crates})
foreach(imported_crate ${imported_crates})
  list(APPEND PLUGIN_BUNDLED_LIBRARIES $<TARGET_FILE:${imported_crate}-shared>)
endforeach()
