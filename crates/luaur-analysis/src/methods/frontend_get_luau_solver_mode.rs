use crate::enums::solver_mode::SolverMode;
use crate::records::frontend::Frontend;
use core::sync::atomic::Ordering;

impl Frontend {
    pub fn get_luau_solver_mode(&self) -> SolverMode {
        match self.use_new_luau_solver.load(Ordering::Relaxed) {
            x if x == SolverMode::Old as i32 => SolverMode::Old,
            _ => SolverMode::New,
        }
    }
}
