extern crate alloc;

use crate::records::json_emitter::JsonEmitter;
use alloc::string::ToString;

pub fn write_json_emitter_long_mut(emitter: &mut JsonEmitter, i: i64) {
    emitter.write_raw_string_view(&i.to_string());
}
