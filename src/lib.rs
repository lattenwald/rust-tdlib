extern crate libc;
use libc::c_double;
use std::os::raw::c_char;

pub enum Tdlib {}

#[link(name = "tdjson")]
extern {
    fn td_json_client_create() -> *mut Tdlib;
    fn td_json_client_send(client: *mut Tdlib, request: *const c_char);
    fn td_json_client_receive(client: *mut Tdlib, timeout: c_double) -> *mut c_char;
    fn td_json_client_execute(client: *mut Tdlib, request: *const c_char) -> *mut c_char;
    fn td_json_client_destroy(client: *mut Tdlib);
}

pub fn client_create() -> *mut Tdlib {
    unsafe {
        td_json_client_create()
    }
}

impl Drop for Tdlib {
    fn drop(&mut self) {
        unsafe {
            td_json_client_destroy(self);
        }
    }
}
