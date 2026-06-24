use crate::records::magic_function::MagicFunction;
use crate::records::magic_function_call_context::MagicFunctionCallContext;
use crate::records::scope::Scope;
use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_call::AstExprCall;

#[derive(Debug, Clone)]
pub struct MagicPcall {
    pub(crate) base: MagicFunction,
    pub(crate) handle_old_solver: fn(
        &mut TypeChecker,
        &std::sync::Arc<Scope>,
        &AstExprCall,
        WithPredicate<TypePackId>,
    ) -> Option<WithPredicate<TypePackId>>,
    pub(crate) infer: fn(&MagicFunctionCallContext) -> bool,
}

impl MagicPcall {
    pub fn handle_old_solver(
        &self,
        context: &mut TypeChecker,
        scope: &std::sync::Arc<Scope>,
        call_site: &AstExprCall,
        old_result: WithPredicate<TypePackId>,
    ) -> Option<WithPredicate<TypePackId>> {
        (self.handle_old_solver)(context, scope, call_site, old_result)
    }

    pub fn infer(&self, ctx: &MagicFunctionCallContext) -> bool {
        (self.infer)(ctx)
    }
}
