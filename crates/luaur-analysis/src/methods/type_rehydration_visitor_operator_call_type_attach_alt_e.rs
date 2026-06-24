use crate::records::any_type::AnyType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    #[inline]
    pub fn operator_call(&mut self, _any: &AnyType) -> *mut AstType {
        let allocator: &mut Allocator = unsafe { &mut *self.allocator };
        let _location = Location::default();
        let name = AstName::new();
        let any_type_ref = AstTypeReference::new(
            Location::default(),
            None,
            name,
            None,
            Location::default(),
            false,
            unsafe { luaur_ast::records::ast_array::AstArray::default() },
        );
        allocator.alloc(any_type_ref) as *mut AstType
    }
}
