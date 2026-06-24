use crate::records::reducer::Reducer;
use alloc::string::String;
use core::ffi::{c_char, c_int, CStr};

extern "C" {
    fn fgets(s: *mut c_char, n: c_int, stream: *mut core::ffi::c_void) -> *mut c_char;
}

impl Reducer {
    pub fn read_line(&self, f: *mut core::ffi::c_void) -> String {
        let mut line = String::new();
        let mut buffer = [0 as c_char; 256];

        unsafe {
            while !fgets(buffer.as_mut_ptr(), buffer.len() as c_int, f).is_null() {
                let c_str = CStr::from_ptr(buffer.as_ptr());
                let bytes = c_str.to_bytes();
                let len = bytes.len();

                if let Ok(s) = core::str::from_utf8(bytes) {
                    line.push_str(s);
                }

                if len > 0 && buffer[len - 1] == b'\n' as c_char {
                    break;
                }
            }
        }

        line
    }
}

pub fn reducer_read_line(this: &Reducer, f: *mut core::ffi::c_void) -> String {
    this.read_line(f)
}
