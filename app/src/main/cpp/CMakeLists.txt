cmake_minimum_required(VERSION 3.22.1)
project("dummy")

if(ANDROID_ABI STREQUAL "arm64-v8a")
    set(LINKER "${ANDROID_TOOLCHAIN_ROOT}/bin/aarch64-linux-android${ANDROID_NATIVE_API_LEVEL}-clang" )
elseif(ANDROID_ABI STREQUAL "armeabi-v7a")
    set(LINKER "${ANDROID_TOOLCHAIN_ROOT}/bin/armv7a-linux-androideabi${ANDROID_NATIVE_API_LEVEL}-clang")
elseif(ANDROID_ABI STREQUAL "x86")
    set(LINKER "${ANDROID_TOOLCHAIN_ROOT}/bin/i686-linux-android${ANDROID_NATIVE_API_LEVEL}-clang" )
elseif(ANDROID_ABI STREQUAL "x86_64")
    set(LINKER "${ANDROID_TOOLCHAIN_ROOT}/bin/x86_64-linux-android${ANDROID_NATIVE_API_LEVEL}-clang" )
endif()

include(FetchContent)
FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.6 # Optionally specify a commit hash, version tag or branch here
        GIT_SHALLOW 1
)
FetchContent_MakeAvailable(Corrosion)
set( Rust_RUSTUP_INSTALL_MISSING_TARGET ON)

corrosion_import_crate( MANIFEST_PATH ${CMAKE_SOURCE_DIR}/rust/Cargo.toml PROFILE dev)
corrosion_set_linker(issue ${LINKER})
add_library(${CMAKE_PROJECT_NAME} SHARED
        dummy.cpp)
add_dependencies(${CMAKE_PROJECT_NAME} issue)
target_link_libraries(${CMAKE_PROJECT_NAME}
        issue
        android
        log)

