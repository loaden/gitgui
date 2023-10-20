#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.3.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_app_run_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "app_run",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(app_run()),
    )
}
fn wire_get_diff_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<mirror_DiffLine>, _>(
        WrapInfo {
            debug_name: "get_diff",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(get_diff()),
    )
}
// Section: wrapper structs

#[derive(Clone)]
pub struct mirror_DiffLine(DiffLine);

#[derive(Clone)]
pub struct mirror_DiffLineType(DiffLineType);

// Section: static checks

const _: fn() = || {
    {
        let DiffLine = None::<DiffLine>.unwrap();
        let _: String = DiffLine.content;
        let _: DiffLineType = DiffLine.line_type;
    }
    match None::<DiffLineType>.unwrap() {
        DiffLineType::None => {}
        DiffLineType::Header => {}
        DiffLineType::Add => {}
        DiffLineType::Delete => {}
    }
};
// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}
// Section: impl IntoDart

impl support::IntoDart for mirror_DiffLine {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0.content.into_into_dart().into_dart(),
            self.0.line_type.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_DiffLine {}
impl rust2dart::IntoIntoDart<mirror_DiffLine> for DiffLine {
    fn into_into_dart(self) -> mirror_DiffLine {
        mirror_DiffLine(self)
    }
}

impl support::IntoDart for mirror_DiffLineType {
    fn into_dart(self) -> support::DartAbi {
        match self.0 {
            DiffLineType::None => 0,
            DiffLineType::Header => 1,
            DiffLineType::Add => 2,
            DiffLineType::Delete => 3,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_DiffLineType {}
impl rust2dart::IntoIntoDart<mirror_DiffLineType> for DiffLineType {
    fn into_into_dart(self) -> mirror_DiffLineType {
        mirror_DiffLineType(self)
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
mod io {
    use super::*;
    // Section: wire functions

    #[no_mangle]
    pub extern "C" fn wire_app_run(port_: i64) {
        wire_app_run_impl(port_)
    }

    #[no_mangle]
    pub extern "C" fn wire_get_diff(port_: i64) {
        wire_get_diff_impl(port_)
    }

    // Section: allocate functions

    // Section: related functions

    // Section: impl Wire2Api

    // Section: wire structs

    // Section: impl NewWithNullPtr

    pub trait NewWithNullPtr {
        fn new_with_null_ptr() -> Self;
    }

    impl<T> NewWithNullPtr for *mut T {
        fn new_with_null_ptr() -> Self {
            std::ptr::null_mut()
        }
    }

    // Section: sync execution mode utility

    #[no_mangle]
    pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
        unsafe {
            let _ = support::box_from_leak_ptr(ptr);
        };
    }
}
#[cfg(not(target_family = "wasm"))]
pub use io::*;
