use crate::functions::write_json_emitter_alt_ae::write_json_emitter_string_view;
use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_i32(emitter: &mut JsonEmitter, i: u32) {
    write_json_emitter_string_view(emitter, &i.to_string());
}
