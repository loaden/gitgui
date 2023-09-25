set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
llvm_path := if os_family() == "windows" { "--llvm-path \"" + replace(replace(trim(`where.exe clang.exe`), "\\bin\\clang.exe", "\""), "\\", "/") } else { "" }
c_path := if os() == "linux" { 'CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d" " -f1 | rev)/include"' } else { "" }

ok:
  echo {{llvm_path}}

run:
  flutter run

gen:
  echo {{c_path}} flutter_rust_bridge_codegen {{llvm_path}}
  {{c_path}} flutter_rust_bridge_codegen {{llvm_path}} \
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
