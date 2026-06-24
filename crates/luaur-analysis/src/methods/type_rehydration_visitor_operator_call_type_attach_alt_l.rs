use crate::functions::get_name_type_attach::get_name_allocator_synthetic_names_generic_type;
use crate::records::generic_type::GenericType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    #[inline]
    pub fn operator_call_7(&mut self, gtv: &GenericType) -> *mut AstType {
        let allocator: &mut Allocator = unsafe { &mut *self.allocator };
        let synthetic_names: &mut crate::type_aliases::synthetic_names::SyntheticNames =
            unsafe { &mut *self.synthetic_names };
        let name_ptr =
            get_name_allocator_synthetic_names_generic_type(allocator, synthetic_names, gtv);
        let name = AstName::ast_name_c_char(name_ptr);
        let type_ref = AstTypeReference::new(
            Location::default(),
            None,
            name,
            None,
            Location::default(),
            false,
            unsafe { luaur_ast::records::ast_array::AstArray::default() },
        );
        allocator.alloc(type_ref) as *mut AstType
    }
}
