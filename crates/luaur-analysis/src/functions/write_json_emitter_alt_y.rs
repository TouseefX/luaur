use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_i32_mut(emitter: &mut JsonEmitter, i: i32) {
    emitter.write_raw_string_view(&i.to_string());
}
