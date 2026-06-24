use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_nullopt_t(emitter: &mut JsonEmitter, _nullopt: ()) {
    emitter.write_raw_string_view("null");
}
