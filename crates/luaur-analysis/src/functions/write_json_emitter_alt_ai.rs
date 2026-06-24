use crate::records::json_emitter::JsonEmitter;
use core::ffi::c_void;

pub fn write_json_emitter_nullptr_t(emitter: &mut JsonEmitter, _null: *const c_void) {
    emitter.write_raw_string_view("null");
}
