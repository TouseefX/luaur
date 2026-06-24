use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct PushTypeConstraint {
    pub(crate) expectedType: TypeId,
    pub(crate) targetType: TypeId,
    pub(crate) astTypes: *const DenseHashMap<*const AstExpr, TypeId>,
    pub(crate) astExpectedTypes: *const DenseHashMap<*const AstExpr, TypeId>,
    pub(crate) expr: *const AstExpr,
}
