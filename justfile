set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
llvm_path := if os_family() == "windows" { "--llvm-path \"" + replace(trim(`where.exe clang`), "\\bin\\clang.exe", "\"") } else { "" }
c_path := if os() == "linux" { 'CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d" " -f1 | rev)/include"' } else { "" }

run:
  flutter run

gen:
  {{c_path}} flutter_rust_bridge_codegen {{llvm_path}} \
    --rust-input rust/src/api.rs \
    --rust-output rust/src/bridge_api.rs \
    --dart-output lib/bridge_api.dart \
    --class-name FfiApi \
    --inline-rust

build:
  cargo build --manifest-path rust/Cargo.toml
  dart run build_runner build

get:
  flutter pub get

upgrade:
	flutter pub upgrade
