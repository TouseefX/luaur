use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::records::unknown_type::UnknownType;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    pub fn operator_call_21(&mut self, _ttv: &UnknownType) -> *mut AstType {
        let allocator = unsafe { &mut *self.allocator };
        let name = AstName::ast_name_c_char(c"unknown".as_ptr());
        let loc = Location::default();
        let empty_params = AstArray {
            data: core::ptr::null_mut(),
            size: 0,
        };
        let reference = AstTypeReference::new(loc, None, name, None, loc, false, empty_params);
        allocator.alloc(reference) as *mut AstType
    }
}
