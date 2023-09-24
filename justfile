set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
llvm_path := if os_family() == "windows" { "--llvm-path D:/Dev/LLVM" } else { "" }

run:
  flutter run

gen:
  export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
  flutter_rust_bridge_codegen {{llvm_path}} \
    --rust-input native/src/api.rs \
    --rust-output native/src/bridge_api.rs \
    --dart-output lib/bridge_api.dart \
    --class-name NativeApi \
    --inline-rust

build:
  cargo build --manifest-path native/Cargo.toml
  dart run build_runner build

get:
  flutter pub get

upgrade:
	flutter pub upgrade
