use crate::records::type_pack_rehydration_visitor::TypePackRehydrationVisitor;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeRehydrationVisitor {
    pub fn rehydrate(&mut self, tp: TypePackId) -> *mut AstTypePack {
        LUAU_ASSERT!(tp != core::ptr::null_mut());

        let type_visitor = self as *mut TypeRehydrationVisitor;
        let tprv =
            TypePackRehydrationVisitor::type_pack_rehydration_visitor_type_pack_rehydration_visitor(
                self.allocator,
                self.synthetic_names,
                type_visitor,
            );

        // C++ `Luau::visit(tprv, tp->ty)` — dispatch over the pack variant.
        tprv.visit_type_pack(tp)
    }
}
