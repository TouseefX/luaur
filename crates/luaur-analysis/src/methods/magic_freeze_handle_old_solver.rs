use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr_call::AstExprCall;

pub fn magic_freeze_handle_old_solver(
    _typechecker: &mut TypeChecker,
    _scope: &ScopePtr,
    _call_site: &AstExprCall,
    _old_result: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
    None
}
