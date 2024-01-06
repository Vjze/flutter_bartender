// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.12.

// Section: imports

use super::*;
use crate::api::simple::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::for_generated::wasm_bindgen;
use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: dart2rust

impl<T> CstDecode<Option<T>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
where
    JsValue: CstDecode<T>,
{
    fn cst_decode(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.cst_decode())
    }
}
impl CstDecode<String> for String {
    fn cst_decode(self) -> String {
        self
    }
}
impl CstDecode<crate::api::simple::DataInfo>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> crate::api::simple::DataInfo {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            10,
            "Expected 10 elements, got {}",
            self_.length()
        );
        crate::api::simple::DataInfo {
            sn: self_.get(0).cst_decode(),
            cus_pn: self_.get(1).cst_decode(),
            sntitle: self_.get(2).cst_decode(),
            in_name: self_.get(3).cst_decode(),
            inloss1: self_.get(4).cst_decode(),
            reloss1: self_.get(5).cst_decode(),
            out_name: self_.get(6).cst_decode(),
            inloss2: self_.get(7).cst_decode(),
            reloss2: self_.get(8).cst_decode(),
            print_num: self_.get(9).cst_decode(),
        }
    }
}
impl CstDecode<crate::api::simple::InitData>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> crate::api::simple::InitData {
        let self_ = self
            .dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::simple::InitData {
            librarie_id: self_.get(0).cst_decode(),
            printers: self_.get(1).cst_decode(),
            btws: self_.get(2).cst_decode(),
            sqlstatus: self_.get(3).cst_decode(),
        }
    }
}
impl CstDecode<Vec<String>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> Vec<String> {
        self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap()
            .iter()
            .map(CstDecode::cst_decode)
            .collect()
    }
}
impl CstDecode<Vec<crate::api::simple::DataInfo>>
    for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(self) -> Vec<crate::api::simple::DataInfo> {
        self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>()
            .unwrap()
            .iter()
            .map(CstDecode::cst_decode)
            .collect()
    }
}
impl CstDecode<Vec<u8>> for Box<[u8]> {
    fn cst_decode(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl
    CstDecode<
        flutter_rust_bridge::RustOpaque<
            std::sync::RwLock<tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
        >,
    > for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
{
    fn cst_decode(
        self,
    ) -> flutter_rust_bridge::RustOpaque<
        std::sync::RwLock<tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
    > {
        unsafe { flutter_rust_bridge::for_generated::cst_decode_rust_opaque(self) }
    }
}
impl CstDecode<String> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl CstDecode<bool> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> bool {
        self.is_truthy()
    }
}
impl CstDecode<i32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<Vec<u8>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> Vec<u8> {
        self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Uint8Array>()
            .to_vec()
            .into()
    }
}
impl CstDecode<u32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<u8> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl CstDecode<usize> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn cst_decode(self) -> usize {
        self.unchecked_into_f64() as _
    }
}

#[wasm_bindgen]
pub fn dart_fn_deliver_output(
    call_id: i32,
    ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
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

#[wasm_bindgen]
pub fn wire_client(port_: flutter_rust_bridge::for_generated::MessagePort, sql: String) {
    wire_client_impl(port_, sql)
}

#[wasm_bindgen]
pub fn wire_do_print(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    sn: String,
    sql: String,
    id: String,
    btw: String,
    printer: String,
    float: u32,
) {
    wire_do_print_impl(port_, sn, sql, id, btw, printer, float)
}

#[wasm_bindgen]
pub fn wire_get_libraries(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_get_libraries_impl(port_)
}

#[wasm_bindgen]
pub fn wire_init_all(port_: flutter_rust_bridge::for_generated::MessagePort, sql: String) {
    wire_init_all_impl(port_, sql)
}

#[wasm_bindgen]
pub fn wire_init_app(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_init_app_impl(port_)
}

#[wasm_bindgen]
pub fn wire_load_btws(port_: flutter_rust_bridge::for_generated::MessagePort, id: String) {
    wire_load_btws_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_load_printers(port_: flutter_rust_bridge::for_generated::MessagePort) {
    wire_load_printers_impl(port_)
}

#[wasm_bindgen]
pub fn wire_run_query(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    sn: String,
    sql: String,
) {
    wire_run_query_impl(port_, sn, sql)
}

#[wasm_bindgen]
pub fn wire_sql_init(port_: flutter_rust_bridge::for_generated::MessagePort, sql: String) {
    wire_sql_init_impl(port_, sql)
}

#[wasm_bindgen]
pub fn wire_updata(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    list: flutter_rust_bridge::for_generated::wasm_bindgen::JsValue,
    sql: String,
) {
    wire_updata_impl(port_, list, sql)
}

#[wasm_bindgen]
pub fn rust_arc_increment_strong_count_RustOpaque_stdsyncRwLocktiberiusClienttokio_utilcompatCompattokionetTcpStream(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        flutter_rust_bridge::for_generated::rust_arc_increment_strong_count::<
            std::sync::RwLock<tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
        >(ptr);
    }
}

#[wasm_bindgen]
pub fn rust_arc_decrement_strong_count_RustOpaque_stdsyncRwLocktiberiusClienttokio_utilcompatCompattokionetTcpStream(
    ptr: *const std::ffi::c_void,
) {
    unsafe {
        flutter_rust_bridge::for_generated::rust_arc_decrement_strong_count::<
            std::sync::RwLock<tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>>,
        >(ptr);
    }
}
