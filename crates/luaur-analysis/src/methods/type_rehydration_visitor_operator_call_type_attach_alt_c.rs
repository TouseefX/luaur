use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    pub fn operator_call_14(&mut self, _petv: &PendingExpansionType) -> *mut AstType {
        let allocator = unsafe { &mut *self.allocator };
        let name = AstName::ast_name_c_char(c"*pending-expansion*".as_ptr());
        let reference = AstTypeReference::new(
            Location::default(),
            None,
            name,
            None,
            Location::default(),
            false,
            luaur_ast::records::ast_array::AstArray::default(),
        );
        allocator.alloc(reference) as *mut AstType
    }
}
