use crate::records::lazy_type::LazyType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    #[inline]
    pub fn operator_call_9(&mut self, ltv: &LazyType) -> *mut AstType {
        // C++ `if (TypeId unwrapped = ltv.unwrapped.load()) return Luau::visit(*this, unwrapped->ty);`
        let unwrapped: TypeId = ltv.unwrapped;
        if !unwrapped.is_null() {
            return self.visit_type(unwrapped);
        }

        let allocator: &mut Allocator = unsafe { &mut *self.allocator };
        let name = AstName::ast_name_c_char(c"<Lazy?>".as_ptr());
        let reference = AstTypeReference::new(
            Location::default(),
            None,
            name,
            None,
            Location::default(),
            false,
            unsafe { luaur_ast::records::ast_array::AstArray::default() },
        );
        allocator.alloc(reference) as *mut AstType
    }
}
