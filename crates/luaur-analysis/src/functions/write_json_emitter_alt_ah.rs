use crate::functions::write_json_emitter_alt_ae::write_json_emitter_string_view;
use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_string(emitter: &mut JsonEmitter, str: &str) {
    write_json_emitter_string_view(emitter, str);
}
