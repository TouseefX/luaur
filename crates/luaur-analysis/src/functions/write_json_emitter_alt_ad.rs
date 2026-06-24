use crate::functions::write_json_emitter_alt_ac::write_json_emitter_long;
use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_long_long(emitter: &mut JsonEmitter, i: u64) {
    write_json_emitter_long(emitter, i);
}
