use crate::records::constraint::Constraint;
use crate::records::dcr_logger::DcrLogger;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::variant::Variant3;

impl DcrLogger {
    pub fn pop_block_type_id(&mut self, block: TypeId) {
        let _block_variant: Variant3<TypeId, TypePackId, *const Constraint> = Variant3::V0(block);
        self.pop_block_not_null_constraint(unsafe {
            core::mem::transmute::<TypeId, *const Constraint>(block)
        });
    }
}
