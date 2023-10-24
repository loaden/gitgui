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

fn wire_get_repo_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "get_repo",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(get_repo()),
    )
}
fn wire_set_repo_impl(
    port_: MessagePort,
    path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "set_repo",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            move |task_callback| Result::<_, ()>::Ok(set_repo(api_path))
        },
    )
}
fn wire_open_default_repo_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "open_default_repo",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(open_default_repo()),
    )
}
fn wire_get_default_repo_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "get_default_repo",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(get_default_repo()),
    )
}
fn wire_fetch_status_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "fetch_status",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(fetch_status()),
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
fn wire_get_status_items_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<String>, _>(
        WrapInfo {
            debug_name: "get_status_items",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(get_status_items()),
    )
}
fn wire_set_status_select_impl(
    port_: MessagePort,
    index: impl Wire2Api<usize> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "set_status_select",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_index = index.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(set_status_select(api_index))
            }
        },
    )
}
fn wire_get_index_items_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<String>, _>(
        WrapInfo {
            debug_name: "get_index_items",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(get_index_items()),
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

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
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
    pub extern "C" fn wire_get_repo(port_: i64) {
        wire_get_repo_impl(port_)
    }

    #[no_mangle]
    pub extern "C" fn wire_set_repo(port_: i64, path: *mut wire_uint_8_list) {
        wire_set_repo_impl(port_, path)
    }

    #[no_mangle]
    pub extern "C" fn wire_open_default_repo(port_: i64) {
        wire_open_default_repo_impl(port_)
    }

    #[no_mangle]
    pub extern "C" fn wire_get_default_repo(port_: i64) {
        wire_get_default_repo_impl(port_)
    }

    #[no_mangle]
    pub extern "C" fn wire_fetch_status(port_: i64) {
        wire_fetch_status_impl(port_)
    }

    #[no_mangle]
    pub extern "C" fn wire_get_diff(port_: i64) {
        wire_get_diff_impl(port_)
    }

    #[no_mangle]
    pub extern "C" fn wire_get_status_items(port_: i64) {
        wire_get_status_items_impl(port_)
    }

    #[no_mangle]
    pub extern "C" fn wire_set_status_select(port_: i64, index: usize) {
        wire_set_status_select_impl(port_, index)
    }

    #[no_mangle]
    pub extern "C" fn wire_get_index_items(port_: i64) {
        wire_get_index_items_impl(port_)
    }

    // Section: allocate functions

    #[no_mangle]
    pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
        let ans = wire_uint_8_list {
            ptr: support::new_leak_vec_ptr(Default::default(), len),
            len,
        };
        support::new_leak_box_ptr(ans)
    }

    // Section: related functions

    // Section: impl Wire2Api

    impl Wire2Api<String> for *mut wire_uint_8_list {
        fn wire2api(self) -> String {
            let vec: Vec<u8> = self.wire2api();
            String::from_utf8_lossy(&vec).into_owned()
        }
    }

    impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
        fn wire2api(self) -> Vec<u8> {
            unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }
        }
    }

    // Section: wire structs

    #[repr(C)]
    #[derive(Clone)]
    pub struct wire_uint_8_list {
        ptr: *mut u8,
        len: i32,
    }

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
