use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_pack_rehydration_visitor::TypePackRehydrationVisitor;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;
use luaur_ast::records::location::Location;

impl TypePackRehydrationVisitor {
    #[inline]
    pub fn operator_call_4(&self, _gtp: &FreeTypePack) -> *mut AstTypePack {
        let allocator = unsafe { &mut *self.allocator };
        let name = AstName {
            value: c"free".as_ptr(),
        };
        let generic = AstTypePackGeneric::new(Location::default(), name);
        allocator.alloc(generic) as *mut AstTypePack
    }
}
