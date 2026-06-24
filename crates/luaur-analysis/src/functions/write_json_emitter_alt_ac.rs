use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_long(emitter: &mut JsonEmitter, i: u64) {
    emitter.write_raw_string_view(&i.to_string());
}
