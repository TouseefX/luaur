use crate::records::constraint::Constraint;
use crate::records::dcr_logger::DcrLogger;
use crate::type_aliases::type_id::TypeId;

impl DcrLogger {
    pub fn push_block_not_null_constraint_type_id(
        &mut self,
        constraint: *const Constraint,
        block: TypeId,
    ) {
        self.push_block_not_null_constraint_type_pack_id(constraint, unsafe {
            core::mem::transmute(block)
        });
    }
}
