use crate::records::constraint::Constraint;
use crate::records::dcr_logger::DcrLogger;

impl DcrLogger {
    pub fn push_block_not_null_constraint_not_null_constraint(
        &mut self,
        constraint: *const Constraint,
        block: *const Constraint,
    ) {
        self.push_block_not_null_constraint_type_id(constraint, unsafe {
            core::mem::transmute(block)
        });
    }
}
