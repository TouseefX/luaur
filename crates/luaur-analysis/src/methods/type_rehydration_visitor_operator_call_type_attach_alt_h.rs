use crate::records::metatable_type::MetatableType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::ast_type::AstType;

impl TypeRehydrationVisitor {
    /// C++ `AstType* operator()(const MetatableType& mtv)` —
    /// `return Luau::visit(*this, mtv.table->ty);`.
    pub fn operator_call_10(&mut self, mtv: &MetatableType) -> *mut AstType {
        self.visit_type(mtv.table())
    }
}
