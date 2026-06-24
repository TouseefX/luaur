use crate::records::constraint::Constraint;
use crate::records::dcr_logger::DcrLogger;
use crate::type_aliases::constraint_block_target::ConstraintBlockTarget;
use luaur_common::records::variant::Variant3;

impl DcrLogger {
    pub fn pop_block_not_null_constraint(&mut self, block: *const Constraint) {
        for (_, list) in self.constraint_blocks.iter_mut() {
            list.retain(|target| {
                if let ConstraintBlockTarget::V2(target_block) = target {
                    *target_block != block
                } else {
                    true
                }
            });
        }
    }
}
