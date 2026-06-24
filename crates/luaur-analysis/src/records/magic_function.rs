use alloc::sync::Arc;
use luaur_ast::records::ast_expr_call::AstExprCall;

use crate::records::magic_function_call_context::MagicFunctionCallContext;
use crate::records::magic_function_type_check_context::MagicFunctionTypeCheckContext;
use crate::records::magic_refinement_context::MagicRefinementContext;
use crate::records::scope::Scope;
use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::type_pack_id::TypePackId;

/// MagicFunction is an abstract base class in C++. In Rust, we model this as a struct
/// containing function pointers (vtable) to allow custom typechecking logic for builtins.
#[derive(Clone)]
pub struct MagicFunction {
    pub(crate) handle_old_solver: fn(
        &mut TypeChecker,
        &Arc<Scope>,
        &AstExprCall,
        WithPredicate<TypePackId>,
    ) -> Option<WithPredicate<TypePackId>>,

    pub(crate) infer: fn(&MagicFunctionCallContext) -> bool,

    pub(crate) refine: fn(&MagicRefinementContext),

    pub(crate) type_check: fn(&MagicFunctionTypeCheckContext) -> bool,
}

impl MagicFunction {
    /// Build a `MagicFunction` vtable from its four handler function pointers.
    /// This is the public analog of constructing a `MagicFunction` subclass
    /// (e.g. the test-only `MagicInstanceIsA`) outside this crate, where the
    /// fields are not directly accessible.
    pub fn from_handlers(
        handle_old_solver: fn(
            &mut TypeChecker,
            &Arc<Scope>,
            &AstExprCall,
            WithPredicate<TypePackId>,
        ) -> Option<WithPredicate<TypePackId>>,
        infer: fn(&MagicFunctionCallContext) -> bool,
        refine: fn(&MagicRefinementContext),
        type_check: fn(&MagicFunctionTypeCheckContext) -> bool,
    ) -> Self {
        MagicFunction {
            handle_old_solver,
            infer,
            refine,
            type_check,
        }
    }
}

impl core::fmt::Debug for MagicFunction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MagicFunction").finish_non_exhaustive()
    }
}

unsafe impl Send for MagicFunction {}
unsafe impl Sync for MagicFunction {}
