use crate::records::blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl BlockedTypeInLiteralVisitor {
    pub fn blocked_type_in_literal_visitor(
        &mut self,
        ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
        to_block: *mut Vec<TypeId>,
    ) {
        self.ast_types = ast_types;
        self.to_block = to_block;
    }
}
