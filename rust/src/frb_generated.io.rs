// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.12.

// Section: imports

use super::*;
use crate::api::simple::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: dart2rust

impl
    CstDecode<
        flutter_rust_bridge::RustOpaque<
            std::sync::RwLock<tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
        >,
    > for *const std::ffi::c_void
{
    fn cst_decode(
        self,
    ) -> flutter_rust_bridge::RustOpaque<
        std::sync::RwLock<tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
    > {
        unsafe { flutter_rust_bridge::for_generated::cst_decode_rust_opaque(self) }
    }
}
impl CstDecode<String> for *mut wire_cst_list_prim_u_8_strict {
    fn cst_decode(self) -> String {
        let vec: Vec<u8> = self.cst_decode();
        String::from_utf8(vec).unwrap()
    }
}
impl CstDecode<crate::api::simple::DataInfo> for wire_cst_data_info {
    fn cst_decode(self) -> crate::api::simple::DataInfo {
        crate::api::simple::DataInfo {
            sn: self.sn.cst_decode(),
            cus_pn: self.cus_pn.cst_decode(),
            sntitle: self.sntitle.cst_decode(),
            in_name: self.in_name.cst_decode(),
            inloss1: self.inloss1.cst_decode(),
            reloss1: self.reloss1.cst_decode(),
            out_name: self.out_name.cst_decode(),
            inloss2: self.inloss2.cst_decode(),
            reloss2: self.reloss2.cst_decode(),
            print_num: self.print_num.cst_decode(),
        }
    }
}
impl CstDecode<crate::api::simple::InitData> for wire_cst_init_data {
    fn cst_decode(self) -> crate::api::simple::InitData {
        crate::api::simple::InitData {
            librarie_id: self.librarie_id.cst_decode(),
            printers: self.printers.cst_decode(),
            btws: self.btws.cst_decode(),
            sqlstatus: self.sqlstatus.cst_decode(),
        }
    }
}
impl CstDecode<Vec<String>> for *mut wire_cst_list_String {
    fn cst_decode(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(CstDecode::cst_decode).collect()
    }
}
impl CstDecode<Vec<crate::api::simple::DataInfo>> for *mut wire_cst_list_data_info {
    fn cst_decode(self) -> Vec<crate::api::simple::DataInfo> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(CstDecode::cst_decode).collect()
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_strict {
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}
impl NewWithNullPtr for wire_cst_data_info {
    fn new_with_null_ptr() -> Self {
        Self {
            sn: core::ptr::null_mut(),
            cus_pn: core::ptr::null_mut(),
            sntitle: core::ptr::null_mut(),
            in_name: core::ptr::null_mut(),
            inloss1: core::ptr::null_mut(),
            reloss1: core::ptr::null_mut(),
            out_name: core::ptr::null_mut(),
            inloss2: core::ptr::null_mut(),
            reloss2: core::ptr::null_mut(),
            print_num: Default::default(),
        }
    }
}
impl Default for wire_cst_data_info {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_cst_init_data {
    fn new_with_null_ptr() -> Self {
        Self {
            librarie_id: core::ptr::null_mut(),
            printers: core::ptr::null_mut(),
            btws: core::ptr::null_mut(),
            sqlstatus: Default::default(),
        }
    }
}
impl Default for wire_cst_init_data {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_dart_fn_deliver_output(
    call_id: i32,
    ptr_: *mut u8,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    let message = unsafe {
        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
            ptr_,
            rust_vec_len_,
            data_len_,
        )
    };
    FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_handle_output(call_id, message)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_client(
    port_: i64,
    sql: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_client_impl(port_, sql)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_do_print(
    port_: i64,
    sn: *mut wire_cst_list_prim_u_8_strict,
    sql: *mut wire_cst_list_prim_u_8_strict,
    id: *mut wire_cst_list_prim_u_8_strict,
    btw: *mut wire_cst_list_prim_u_8_strict,
    printer: *mut wire_cst_list_prim_u_8_strict,
    float: u32,
) {
    wire_do_print_impl(port_, sn, sql, id, btw, printer, float)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_get_libraries(port_: i64) {
    wire_get_libraries_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_init_all(
    port_: i64,
    sql: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_init_all_impl(port_, sql)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_init_app(port_: i64) {
    wire_init_app_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_load_btws(
    port_: i64,
    id: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_load_btws_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_load_printers(port_: i64) {
    wire_load_printers_impl(port_)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_run_query(
    port_: i64,
    sn: *mut wire_cst_list_prim_u_8_strict,
    sql: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_run_query_impl(port_, sn, sql)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_sql_init(
    port_: i64,
    sql: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_sql_init_impl(port_, sql)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_wire_updata(
    port_: i64,
    list: *mut wire_cst_list_data_info,
    sql: *mut wire_cst_list_prim_u_8_strict,
) {
    wire_updata_impl(port_, list, sql)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_rust_arc_increment_strong_count_RustOpaque_stdsyncRwLocktiberiusClienttokio_utilcompatCompattokionetTcpStream(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        flutter_rust_bridge::for_generated::rust_arc_increment_strong_count::<
            std::sync::RwLock<tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
        >(ptr);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLocktiberiusClienttokio_utilcompatCompattokionetTcpStream(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        flutter_rust_bridge::for_generated::rust_arc_decrement_strong_count::<
            std::sync::RwLock<tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
        >(ptr);
    }
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_cst_new_list_String(
    len: i32,
) -> *mut wire_cst_list_String {
    let wrap = wire_cst_list_String {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(
            <*mut wire_cst_list_prim_u_8_strict>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_cst_new_list_data_info(
    len: i32,
) -> *mut wire_cst_list_data_info {
    let wrap = wire_cst_list_data_info {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(
            <wire_cst_data_info>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_bartender_cst_new_list_prim_u_8_strict(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_strict {
    let ans = wire_cst_list_prim_u_8_strict {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_data_info {
    sn: *mut wire_cst_list_prim_u_8_strict,
    cus_pn: *mut wire_cst_list_prim_u_8_strict,
    sntitle: *mut wire_cst_list_prim_u_8_strict,
    in_name: *mut wire_cst_list_prim_u_8_strict,
    inloss1: *mut wire_cst_list_prim_u_8_strict,
    reloss1: *mut wire_cst_list_prim_u_8_strict,
    out_name: *mut wire_cst_list_prim_u_8_strict,
    inloss2: *mut wire_cst_list_prim_u_8_strict,
    reloss2: *mut wire_cst_list_prim_u_8_strict,
    print_num: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_init_data {
    librarie_id: *mut wire_cst_list_prim_u_8_strict,
    printers: *mut wire_cst_list_String,
    btws: *mut wire_cst_list_String,
    sqlstatus: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_String {
    ptr: *mut *mut wire_cst_list_prim_u_8_strict,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_data_info {
    ptr: *mut wire_cst_data_info,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_strict {
    ptr: *mut u8,
    len: i32,
}
