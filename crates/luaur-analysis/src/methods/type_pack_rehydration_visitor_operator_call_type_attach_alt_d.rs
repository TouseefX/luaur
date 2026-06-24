use crate::records::type_pack_rehydration_visitor::TypePackRehydrationVisitor;
use crate::records::variadic_type_pack::VariadicTypePack;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;
use luaur_ast::records::location::Location;

impl TypePackRehydrationVisitor {
    #[inline]
    pub fn operator_call_8(&self, vtp: &VariadicTypePack) -> *mut AstTypePack {
        if vtp.hidden {
            return core::ptr::null_mut();
        }

        // C++ `Luau::visit(*typeVisitor, vtp.ty->ty)`.
        let type_visitor = unsafe { &mut *self.type_visitor };
        let variadic_type = type_visitor.visit_type(vtp.ty);

        let allocator = unsafe { &mut *self.allocator };
        let node = AstTypePackVariadic::new(Location::default(), variadic_type);
        allocator.alloc(node) as *mut AstTypePack
    }
}
