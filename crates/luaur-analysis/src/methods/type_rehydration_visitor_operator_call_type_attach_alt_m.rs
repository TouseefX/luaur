use crate::records::bound::Bound;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_type::AstType;

impl TypeRehydrationVisitor {
    /// C++ `AstType* operator()(const Unifiable::Bound<TypeId>& bound)` —
    /// `return Luau::visit(*this, bound.boundTo->ty);`.
    pub fn operator_call_19(&mut self, bound: &Bound<TypeId>) -> *mut AstType {
        self.visit_type(bound.boundTo)
    }
}
