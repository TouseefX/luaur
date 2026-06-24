use crate::enums::solver_mode::SolverMode;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::module::Module;
use crate::records::substitution::Substitution;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

#[derive(Debug, Clone)]
pub struct ClonePublicInterface {
    pub(crate) base: Substitution,
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) module: *mut Module,
    pub(crate) solver_mode: SolverMode,
    pub(crate) internal_type_escaped: bool,
}
