//! @interface-stub
use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;
use luaur_ast::records::ast_name::AstName;

#[derive(Debug, Clone)]
pub struct TypeFunctionContext {
    pub arena: NonNull<TypeArena>,
    pub builtins: NonNull<BuiltinTypes>,
    pub scope: NonNull<Scope>,
    pub normalizer: NonNull<Normalizer>,
    pub type_function_runtime: NonNull<TypeFunctionRuntime>,
    pub ice: NonNull<InternalErrorReporter>,
    pub limits: NonNull<TypeCheckLimits>,
    pub subtyping: NonNull<Subtyping>,
    pub solver: *mut ConstraintSolver,
    pub constraint: *const Constraint,
    pub user_func_name: Option<AstName>,
    pub fresh_instances: alloc::vec::Vec<TypeId>,
}

// NOTE: the two ctors and `push_constraint` are implemented in their own
// method files (so this record stays a pure data definition):
//   - methods/type_function_context_type_function_context_type_function.rs
//     (`from_components` — the arena/builtins/... ctor, TypeFunction.h:60)
//   - methods/type_function_context_type_function_context_builtin_type_functions.rs
//     (`from_solver` — the ConstraintSolver ctor, BuiltinTypeFunctions.cpp:362)
//   - methods/type_function_context_push_constraint.rs (`push_constraint`)
