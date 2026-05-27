# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html.
# For more examples on how to use CMake, see https://github.com/android/ndk-samples.

# Sets the minimum CMake version required for this project.
cmake_minimum_required(VERSION 3.22.1)

# Declares the project name. The project name can be accessed via ${ PROJECT_NAME},
# Since this is the top level CMakeLists.txt, the project name is also accessible
# with ${CMAKE_PROJECT_NAME} (both CMake variables are in-sync within the top level
# build script scope).
project("dummy")

# Creates and names a library, sets it as either STATIC
# or SHARED, and provides the relative paths to its source code.
# You can define multiple libraries, and CMake builds them for you.
# Gradle automatically packages shared libraries with your APK.
#
# In this top level CMakeLists.txt, ${CMAKE_PROJECT_NAME} is used to define
# the target library name; in the sub-module's CMakeLists.txt, ${PROJECT_NAME}
# is preferred for the same purpose.
#
# In order to load a library into your app from Java/Kotlin, you must call
# System.loadLibrary() and pass the name of the library defined here;
# for GameActivity/NativeActivity derived applications, the same library name must be
# used in the AndroidManifest.xml file.
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

corrosion_import_crate( MANIFEST_PATH ${CMAKE_SOURCE_DIR}/rust/Cargo.toml)
corrosion_set_linker(issue ${LINKER})
# for simpler linkage purposes
add_library(${CMAKE_PROJECT_NAME} SHARED
        dummy.cpp)
add_dependencies(${CMAKE_PROJECT_NAME} issue)
target_link_libraries(${CMAKE_PROJECT_NAME}
        issue
        android
        log)

