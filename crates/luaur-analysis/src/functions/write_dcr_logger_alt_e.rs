use crate::functions::to_pointer_id_dcr_logger_alt_b::to_pointer_id_not_null_constraint;
use crate::functions::write_dcr_logger::write_json_emitter_t;
use crate::records::constraint::Constraint;
use crate::records::json_emitter::JsonEmitter;

pub fn write_json_emitter_not_null_constraint(emitter: &mut JsonEmitter, ptr: *const Constraint) {
    let tmp = to_pointer_id_not_null_constraint(ptr);
    write_json_emitter_t(emitter, tmp.as_ptr());
}
