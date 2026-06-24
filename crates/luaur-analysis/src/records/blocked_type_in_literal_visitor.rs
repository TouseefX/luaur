use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct BlockedTypeInLiteralVisitor {
    pub(crate) ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
    pub(crate) to_block: *mut Vec<TypeId>,
}
