use crate::functions::get_name_type_attach_alt_c::get_name_allocator_synthetic_names_generic_type_pack;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_pack_rehydration_visitor::TypePackRehydrationVisitor;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;
use luaur_ast::records::location::Location;

impl TypePackRehydrationVisitor {
    #[inline]
    pub fn operator_call_5(&self, gtp: &GenericTypePack) -> *mut AstTypePack {
        let allocator = unsafe { &mut *self.allocator };
        let synthetic_names = unsafe { &mut *self.synthetic_names };
        let name_ptr =
            get_name_allocator_synthetic_names_generic_type_pack(allocator, synthetic_names, gtp);
        let name = unsafe { AstName { value: name_ptr } };
        let generic = AstTypePackGeneric::new(Location::default(), name);
        allocator.alloc(generic) as *mut AstTypePack
    }
}
