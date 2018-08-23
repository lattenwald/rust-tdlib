extern crate libc;
use libc::{c_void, c_double};
use std::os::raw::c_char;


#[link(name = "tdjson")]
extern {
    fn td_json_client_create() -> *mut c_void;
    fn td_json_client_send(client: *mut c_void, request: *const c_char);
    fn td_json_client_receive(client: *mut c_void, timeout: c_double) -> *mut c_char;
    fn td_json_client_execute(client: *mut c_void, request: *const c_char) -> *mut c_char;
    fn td_json_client_destroy(client: *mut c_void);
}

fn main() {
    println!("Hello, world!");
}
