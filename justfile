set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
llvm_path := if os_family() == "windows" { "--llvm-path \"" + replace(trim(`where.exe clang`), "\\bin\\clang.exe", "\"") } else { "" }
c_path := if os() == "linux" { 'CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d" " -f1 | rev)/include"' } else { "" }

run:
  flutter run

gen:
  {{c_path}} flutter_rust_bridge_codegen {{llvm_path}} \
    --rust-input ffi/src/api.rs \
    --rust-output ffi/src/bridge_api.rs \
    --dart-output lib/bridge_api.dart \
    --c-output ios/Runner/bridge_api.h \
    --extra-c-output-path macos/Runner/ \
    --class-name FfiApi \
    --dart-format-line-length 80 \
    --inline-rust

build:
  cargo build --manifest-path ffi/Cargo.toml
  dart run build_runner build

get:
  flutter pub get

upgrade:
	flutter pub upgrade
