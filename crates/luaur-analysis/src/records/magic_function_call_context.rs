use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::type_aliases::type_pack_id::TypePackId;
use core::ptr::NonNull;
use luaur_ast::records::ast_expr_call::AstExprCall;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct MagicFunctionCallContext {
    pub solver: NonNull<ConstraintSolver>,
    pub constraint: NonNull<Constraint>,
    pub call_site: NonNull<AstExprCall>,
    pub arguments: TypePackId,
    pub result: TypePackId,
}
