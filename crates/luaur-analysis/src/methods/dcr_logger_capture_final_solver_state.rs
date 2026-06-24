use crate::records::constraint::Constraint;
use crate::records::dcr_logger::DcrLogger;
use crate::records::scope::Scope;
use alloc::vec::Vec;

impl DcrLogger {
    pub fn capture_final_solver_state(
        &mut self,
        root_scope: &Scope,
        unsolved_constraints: &Vec<*const Constraint>,
    ) {
        let mut target = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        self.capture_boundary_state(&mut target, root_scope, unsolved_constraints);
    }
}
