set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
llvm_path := if os_family() == "windows" { "--llvm-path \"" + replace(trim(`where.exe clang`), "\\bin\\clang.exe", "\"") } else { "" }
c_path := if os() == "linux" { 'CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d" " -f1 | rev)/include"' } else { "" }
target := if os() == "macos" { "macos" } else if os() == "windows" { "windows" } else { "linux" }
export BUILD_TYPE := if os_family() == "windows" { "Debug" } else { "" }
export ANDROID_TARGET_ABI := env_var_or_default('ANDROID_TARGET_ABI', 'x86_64')

default:
  flutter run

run:
  flutter run -d {{target}}

build $BUILD_TYPE="Release":
  flutter build {{target}}

release $BUILD_TYPE="Release":
  flutter run --release

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
