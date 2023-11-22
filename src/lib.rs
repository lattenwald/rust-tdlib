extern crate libc;
#[macro_use]
extern crate log;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};

#[derive(Debug, Clone)]
pub struct Tdlib {
    inner: i32,
}

#[link(name = "tdjson")]
extern "C" {
    fn td_create_client_id() -> c_int;
    fn td_send(client_id: c_int, request: *const c_char);
    fn td_execute(request: *const c_char) -> *const c_char;
    fn td_receive(timeout: c_double) -> *const c_char;
}

impl Tdlib {
    /// Creates a new instance of TDLib.
    ///
    /// # Examples
    ///
    /// ```
    /// let tdlib = Tdlib::new();
    /// ```
    pub fn new() -> Self {
        debug!("creating tdlib");
        let client = unsafe { td_create_client_id() };
        Tdlib { inner: client }
    }

    /// Sends request to the TDLib client.
    ///
    /// May be called from any thread.
    ///
    /// # Examples
    ///
    /// ```
    /// let request = r#"{"@type": "getMe"}"#;
    /// tdlib.send(request);
    /// ```
    pub fn send(&self, request: &str) {
        debug!("tdlib send: {}", request);
        let cstring = CString::new(request).unwrap();
        unsafe { td_send(self.inner, cstring.as_ptr()) }
    }

    /// Synchronously executes TDLib request.
    ///
    /// May be called from any thread. Only a few requests can be executed synchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// let request = r#"{"@type": "getTextEntities", "text": "@telegram /test_command https://telegram.org telegram.me"}"#;
    /// tdlib::execute(request);
    /// ```
    pub fn execute(request: &str) -> Option<String> {
        debug!("tdlib execute: {}", request);
        let cstring = CString::new(request).unwrap();
        let result = unsafe {
            td_execute(cstring.as_ptr())
                .as_ref()
                .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
        };
        debug!("tdlib execute result: {:?}", result);
        result
    }

    /// Receives incoming updates and request responses from the TDLib client.
    ///
    /// May be called from any thread, but shouldn't be called simultaneously
    /// from two different threads.
    ///
    /// # Examples
    ///
    /// ```
    /// tdlib.receive(5.0);
    /// ```
    pub fn receive(&self, timeout: f64) -> Option<String> {
        debug!("tdlib receive with timeout {}s", timeout);
        unsafe {
            match td_receive(timeout)
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

impl Default for Tdlib {
    fn default() -> Self {
        Self::new()
    }
}
