use crate::enums::solver_mode::SolverMode;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::source_module::SourceModule;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use core::ptr::NonNull;

#[derive(Debug)]
pub struct GlobalTypes {
    pub(crate) builtin_types: NonNull<BuiltinTypes>,
    pub(crate) global_types: TypeArena,
    pub(crate) global_names: SourceModule,
    pub(crate) global_scope: ScopePtr,
    pub(crate) global_type_function_scope: ScopePtr,
    pub(crate) mode: SolverMode,
}
