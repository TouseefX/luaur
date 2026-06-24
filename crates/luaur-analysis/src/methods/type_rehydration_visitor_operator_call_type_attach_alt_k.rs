use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::error_type::ErrorType;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    pub fn operator_call_3(&mut self, _err: &ErrorType) -> *mut AstType {
        let allocator = unsafe { &mut *self.allocator };
        let name = AstName::ast_name_c_char(c"Unifiable<Error>".as_ptr());
        let node = AstTypeReference::new(
            Location::default(),
            None,
            name,
            None,
            Location::default(),
            false,
            Default::default(),
        );
        allocator.alloc(node) as *mut AstType
    }
}
