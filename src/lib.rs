extern crate libc;
#[macro_use]
extern crate log;

use libc::{c_double, c_void};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

type TdlibClient = *mut c_void;

#[derive(Debug)]
pub struct Tdlib {
    instance: TdlibClient,
}

#[link(name = "tdjson")]
extern "C" {
    fn td_json_client_create() -> TdlibClient;
    fn td_json_client_send(client: TdlibClient, request: *const c_char);
    fn td_json_client_receive(client: TdlibClient, timeout: c_double) -> *mut c_char;
    fn td_json_client_execute(client: TdlibClient, request: *const c_char) -> *mut c_char;
    fn td_json_client_destroy(client: TdlibClient);
}

impl Tdlib {
    pub fn new() -> Self {
        debug!("creating tdlib");
        let client = unsafe { td_json_client_create() };
        Tdlib { instance: client }
    }

    pub fn send(&self, request: &str) {
        debug!("tdlib send: {}", request);
        let cstring = CString::new(request).unwrap();
        unsafe { td_json_client_send(self.instance, cstring.as_ptr()) }
    }

    pub fn execute(&self, request: &str) -> String {
        debug!("tdlib execute: {}", request);
        let cstring = CString::new(request).unwrap();
        let result = unsafe {
            let response = td_json_client_execute(self.instance, cstring.as_ptr());
            CStr::from_ptr(response).to_string_lossy().into_owned()
        };
        debug!("tdlib execute result: {}", result);
        result
    }

    pub fn receive(&self, timeout: f64) -> Option<String> {
        debug!("tdlib receive with timeout {}s", timeout);
        unsafe {
            match td_json_client_receive(self.instance, timeout)
                .as_ref()
                .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
            {
                None => {
                    debug!("tdlib receive timeout");
                    None
                }
                Some(contents) => {
                    debug!("tdlib receive result: {}", contents);
                    Some(contents)
                }
            }
        }
    }
}

impl Drop for Tdlib {
    fn drop(&mut self) {
        debug!("destroying tdlib");
        unsafe {
            td_json_client_destroy(self.instance);
        }
    }
}
