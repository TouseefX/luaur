use crate::records::builtin_types::BuiltinTypes;
use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ExpectedTypeVisitor {
    pub fn new(
        ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
        ast_expected_types: *mut DenseHashMap<*const AstExpr, TypeId>,
        ast_resolved_types: *mut DenseHashMap<*const AstType, TypeId>,
        ast_overload_resolved_types: *mut DenseHashMap<*const AstNode, TypeId>,
        arena: *mut TypeArena,
        builtin_types: *mut BuiltinTypes,
        root_scope: *mut Scope,
    ) -> Self {
        Self {
            ast_types,
            ast_expected_types,
            ast_resolved_types,
            ast_overload_resolved_types,
            arena,
            builtin_types,
            root_scope,
        }
    }
}
