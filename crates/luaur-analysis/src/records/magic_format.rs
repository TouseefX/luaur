use crate::records::magic_function::MagicFunction;
use crate::records::magic_function_call_context::MagicFunctionCallContext;
use crate::records::magic_function_type_check_context::MagicFunctionTypeCheckContext;
use crate::records::scope::Scope;
use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_call::AstExprCall;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MagicFormat {
    pub base: MagicFunction,
}

impl MagicFormat {
    pub(crate) fn handle_old_solver(
        _checker: &mut TypeChecker,
        _scope: &Arc<Scope>,
        _call_site: &AstExprCall,
        _old_result: WithPredicate<TypePackId>,
    ) -> Option<WithPredicate<TypePackId>> {
        None
    }

    pub(crate) fn infer(_ctx: &MagicFunctionCallContext) -> bool {
        false
    }

    pub(crate) fn type_check(_ctx: &MagicFunctionTypeCheckContext) -> bool {
        false
    }
}
