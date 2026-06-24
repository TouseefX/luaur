use crate::functions::write_json_emitter_alt_ae::write_json_emitter_string_view;
use crate::records::json_emitter::JsonEmitter;
use core::ffi::c_char;
use core::ffi::CStr;

pub fn write_json_emitter_c_char(emitter: &mut JsonEmitter, str: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(str) };
    if let Ok(s) = c_str.to_str() {
        write_json_emitter_string_view(emitter, s);
    }
}
