use crate::records::constraint::Constraint;
use crate::records::constraint_block::ConstraintBlock;
use crate::records::constraint_generation_log::ConstraintGenerationLog;
use crate::records::to_string_options::ToStringOptions;
use crate::records::type_check_log::TypeCheckLog;
use crate::records::type_solve_log::TypeSolveLog;
use crate::type_aliases::constraint_block_target::ConstraintBlockTarget;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct DcrLogger {
    pub(crate) generation_log: ConstraintGenerationLog,
    pub(crate) constraint_blocks:
        DenseHashMap<*const Constraint, alloc::vec::Vec<ConstraintBlockTarget>>,
    pub(crate) solve_log: TypeSolveLog,
    pub(crate) check_log: TypeCheckLog,
    pub(crate) opts: ToStringOptions,
}
