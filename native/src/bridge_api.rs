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
// Generated by `flutter_rust_bridge`@ 1.82.1.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_times_from_rust_impl(
    port_: MessagePort,
    left: impl Wire2Api<usize> + UnwindSafe,
    right: impl Wire2Api<usize> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, usize, _>(
        WrapInfo {
            debug_name: "times_from_rust",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_left = left.wire2api();
            let api_right = right.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(times_from_rust(api_left, api_right))
            }
        },
    )
}
fn wire_hello_from_rust_impl(
    port_: MessagePort,
    count: impl Wire2Api<usize> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "hello_from_rust",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_count = count.wire2api();
            move |task_callback| Result::<_, ()>::Ok(hello_from_rust(api_count))
        },
    )
}
fn wire_get_diff_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "get_diff",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(get_diff()),
    )
}
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
// Section: wrapper structs

// Section: static checks

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
impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
    }
}
// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
mod io {
    use super::*;
    // Section: wire functions

    #[no_mangle]
    pub extern "C" fn wire_times_from_rust(
        port_: i64,
        left: usize,
        right: usize,
    ) {
        wire_times_from_rust_impl(port_, left, right)
    }

    #[no_mangle]
    pub extern "C" fn wire_hello_from_rust(port_: i64, count: usize) {
        wire_hello_from_rust_impl(port_, count)
    }

    #[no_mangle]
    pub extern "C" fn wire_get_diff(port_: i64) {
        wire_get_diff_impl(port_)
    }

    #[no_mangle]
    pub extern "C" fn wire_app_run(port_: i64) {
        wire_app_run_impl(port_)
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
