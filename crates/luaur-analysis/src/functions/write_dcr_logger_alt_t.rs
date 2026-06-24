use crate::functions::write_dcr_logger_alt_r::write_json_emitter_constraint_step_snapshot;
use crate::functions::write_dcr_logger_alt_s::write_json_emitter_generalize_step_snapshot;
use crate::records::json_emitter::JsonEmitter;
use crate::type_aliases::step_snapshot::StepSnapshot;
use luaur_common::records::variant::Variant2;

pub fn write_json_emitter_step_snapshot(emitter: &mut JsonEmitter, snap: &StepSnapshot) {
    match snap {
        Variant2::V0(s) => write_json_emitter_constraint_step_snapshot(emitter, s),
        Variant2::V1(s) => write_json_emitter_generalize_step_snapshot(emitter, s),
    }
}
