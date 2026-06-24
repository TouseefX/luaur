use crate::records::type_pack_rehydration_visitor::TypePackRehydrationVisitor;
use crate::type_aliases::error_type_pack::ErrorTypePack;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;
use luaur_ast::records::location::Location;

impl TypePackRehydrationVisitor {
    #[inline]
    pub fn operator_call_3(&self, _tp: &ErrorTypePack) -> *mut AstTypePack {
        let allocator = unsafe { &mut *self.allocator };
        let name = AstName {
            value: c"Unifiable<Error>".as_ptr(),
        };
        let generic = AstTypePackGeneric::new(Location::default(), name);
        allocator.alloc(generic) as *mut AstTypePack
    }
}
