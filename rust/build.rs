use std::env;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};

// Path of input Rust code
const RUST_INPUT: &str = "src/api.rs";
// Path of output generated Dart code
const DART_OUTPUT: &str = "../lib/bridge_api.dart";
// Path of output Rust code
const RUST_OUTPUT: &str = "src/bridge_api.rs";
// Class name to use in dart, corresponding to each Rust block
const CLASS_NAME: &str = "FfiApi";

fn main() {
    // Tell Cargo that if the input Rust code changes, to rerun this build script.
    println!("cargo:rerun-if-changed={RUST_INPUT}");

    // Options for frb_codegen
    let raw_opts = RawOpts {
        // Path of input Rust code
        rust_input: vec![RUST_INPUT.to_string()],
        // Path of output generated Dart code
        dart_output: vec![DART_OUTPUT.to_string()],
        // Path of output Rust code
        rust_output: Some(vec![RUST_OUTPUT.to_string()]),
        // Class name of each Rust block of api
        class_name: Some(vec![CLASS_NAME.to_string()]),
        // Other options
        llvm_path: Some(vec![llvm_path()]),
        dart_format_line_length: 80,
        inline_rust: true,
        wasm: false,
        // for other options use defaults
        ..Default::default()
    };

    // set CPATH: export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
    // https://cjycode.com/flutter_rust_bridge/integrate/deps.html
    if cfg!(target_os = "linux") {
        env::set_var("CPATH", c_path());
    }

    // get opts from raw opts
    let all_configs = config_parse(raw_opts);
    // generation of rust api for ffi
    let all_symbols = get_symbols_if_no_duplicates(&all_configs).unwrap();
    frb_codegen(&all_configs[0], &all_symbols).unwrap();
}

fn llvm_path() -> String {
    if cfg!(target_os = "windows") {
        let mut str = String::new();
        if let Ok(mut child) = Command::new("clang")
            .arg("-v")
            .stderr(Stdio::piped())
            .spawn()
        {
            let mut stderr = child.stderr.take().unwrap();
            stderr.read_to_string(&mut str).unwrap();
            let v: Vec<_> = str.split("InstalledDir:").collect();
            let mut path = PathBuf::from(v[1].trim());
            path.pop();
            if let Some(s) = path.to_str() {
                str = s.into();
            }
        }
        str
    } else {
        "".to_string()
    }
}

fn c_path() -> String {
    let mut str = String::new();
    if let Ok(mut child) = Command::new("clang")
        .arg("-v")
        .stderr(Stdio::piped())
        .spawn()
    {
        let mut stderr = child.stderr.take().unwrap();
        stderr.read_to_string(&mut str).unwrap();
        let v: Vec<_> = str.split("Selected GCC installation:").collect();
        for line in v[1].lines() {
            let mut path = PathBuf::from(line.trim());
            path.push("include");
            if let Some(s) = path.to_str() {
                return s.to_string();
            }
        }
    }
    str
}
