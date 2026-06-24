use crate::records::type_pack_rehydration_visitor::TypePackRehydrationVisitor;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl TypePackRehydrationVisitor {
    /// C++ `AstTypePack* operator()(const BoundTypePack& btp) const` —
    /// `return Luau::visit(*this, btp.boundTo->ty);`.
    pub fn operator_call_2(&self, btp: &BoundTypePack) -> *mut AstTypePack {
        self.visit_type_pack(btp.boundTo)
    }
}
