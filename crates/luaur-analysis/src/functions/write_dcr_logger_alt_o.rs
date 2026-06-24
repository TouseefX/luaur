use crate::functions::to_pointer_id_dcr_logger::to_pointer_id;
use crate::functions::to_pointer_id_dcr_logger_alt_b::to_pointer_id_not_null_constraint;
use crate::functions::write_dcr_logger::write_json_emitter_t;
use crate::functions::write_dcr_logger_alt_e::write_json_emitter_not_null_constraint;
use crate::records::constraint_block::ConstraintBlock;
use crate::records::json_emitter::JsonEmitter;
use crate::records::object_emitter::ObjectEmitter;
use crate::type_aliases::constraint_block_target::ConstraintBlockTarget;
use luaur_common::functions::visit_variant::visit;
use luaur_common::records::variant::Variant2;

pub fn write_json_emitter_constraint_block(emitter: &mut JsonEmitter, block: &ConstraintBlock) {
    let mut o = emitter.write_object();
    o.write_pair("stringification", &block.stringification);

    let target = &block.target;

    let kind = match target {
        ConstraintBlockTarget::V0(_) => "type",
        ConstraintBlockTarget::V1(_) => "typePack",
        ConstraintBlockTarget::V2(_) => "constraint",
    };

    let ptr_id = match target {
        ConstraintBlockTarget::V0(ty) => to_pointer_id(*ty),
        ConstraintBlockTarget::V1(tp) => to_pointer_id(*tp),
        ConstraintBlockTarget::V2(c) => to_pointer_id_not_null_constraint(*c),
    };

    o.write_pair("id", &ptr_id);
    o.write_pair("kind", kind);

    o.finish();
}
