use crate::records::no_refine_type::NoRefineType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    #[inline]
    pub fn operator_call_13(&mut self, _no_refine: &NoRefineType) -> *mut AstType {
        let allocator = unsafe { &mut *self.allocator };
        let name = AstName::ast_name_c_char(c"*no-refine*".as_ptr());
        let reference = AstTypeReference::new(
            Location::default(),
            None,
            name,
            None,
            Location::default(),
            false,
            Default::default(),
        );
        allocator.alloc(reference) as *mut AstType
    }
}
