use crate::functions::write_json_emitter_alt_ad::write_json_emitter_long_long;
use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_long_long_mut(emitter: &mut JsonEmitter, i: i64) {
    write_json_emitter_long_long(emitter, i as u64);
}
