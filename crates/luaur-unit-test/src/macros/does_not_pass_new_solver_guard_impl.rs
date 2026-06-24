#[macro_export]
#[allow(non_snake_case)]
macro_rules! DOES_NOT_PASS_NEW_SOLVER_GUARD_IMPL {
    ($line:expr) => {
        let _sff = crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
            &luaur_common::FFlag::DebugLuauForceOldSolver,
            !luaur_common::FFlag::DebugLuauForceAllNewSolverTests.get(),
        );
    };
}

pub use DOES_NOT_PASS_NEW_SOLVER_GUARD_IMPL;
