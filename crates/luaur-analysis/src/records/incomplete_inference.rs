use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct IncompleteInference {
    pub(crate) expectedType: TypeId,
    pub(crate) targetType: TypeId,
    pub(crate) expr: *const AstExpr,
}
