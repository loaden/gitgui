set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
llvm_path := if os_family() == "windows" { "--llvm-path \"" + replace(trim(`where.exe clang`), "\\bin\\clang.exe", "\"") } else { "" }
c_path := if os() == "linux" { 'CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d" " -f1 | rev)/include"' } else { "" }
target := if os() == "macos" { "macos" } else if os() == "windows" { "windows" } else { "linux" }
export BUILD_TYPE := if os_family() == "windows" { "Debug" } else { "" }

# Android
export ANDROID_ABI_DEBUG := env_var_or_default('ANDROID_ABI_DEBUG', 'x86_64')
export OPENSSL_DIR := if os_family() == "windows" { "C:/Users/Lucky/.dev/sdk/openssl/" } else { "" }
export AARCH64_LINUX_ANDROID_OPENSSL_LIB_DIR := OPENSSL_DIR + "arm64-v8a"
export X86_64_LINUX_ANDROID_OPENSSL_LIB_DIR := OPENSSL_DIR + "x86_64"
export ARMV7_LINUX_ANDROIDEABI_OPENSSL_LIB_DIR := OPENSSL_DIR + "armeabi-v7a"


default:
  flutter run

arg arg:
  flutter run {{arg}}

run:
  flutter run -d {{target}}

build $BUILD_TYPE="Release":
  flutter build {{target}}

release $BUILD_TYPE="Release":
  flutter run --release -d {{target}}

android $ANDROID_ABI_DEBUG="":
  flutter build apk

gen:
  {{c_path}} flutter_rust_bridge_codegen {{llvm_path}} \
    --rust-input native/src/api.rs \
    --rust-output native/src/bridge_api.rs \
    --dart-output lib/bridge_api.dart \
    --c-output ios/Runner/bridge_api.h \
    --extra-c-output-path macos/Runner/ \
    --class-name Native \
    --dart-format-line-length 80 \
    --inline-rust

get:
  flutter pub get

upgrade:
	flutter pub upgrade
