use crate::enums::solver_mode::SolverMode;
use crate::records::frontend::Frontend;
use core::sync::atomic::Ordering;

impl Frontend {
    pub fn set_luau_solver_mode(&mut self, mode: SolverMode) {
        self.use_new_luau_solver
            .store(mode as i32, Ordering::Relaxed);
    }
}
