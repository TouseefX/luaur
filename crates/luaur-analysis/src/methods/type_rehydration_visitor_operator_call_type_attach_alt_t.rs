use crate::records::negation_type::NegationType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_or_pack::AstTypeOrPack;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    #[inline]
    pub fn operator_call_11(&mut self, ntv: &NegationType) -> *mut AstType {
        // C++ `params.data[0] = AstTypeOrPack{Luau::visit(*this, ntv.ty->ty), nullptr};`
        let ty_rehydrated = self.visit_type(ntv.ty);

        let allocator: &mut Allocator = unsafe { &mut *self.allocator };

        let params_data: *mut AstTypeOrPack =
            allocator.allocate(std::mem::size_of::<AstTypeOrPack>()) as *mut AstTypeOrPack;
        unsafe {
            *params_data = AstTypeOrPack {
                r#type: ty_rehydrated,
                type_pack: std::ptr::null_mut(),
            };
        }

        let params = luaur_ast::records::ast_array::AstArray {
            data: params_data,
            size: 1,
        };

        let reference = AstTypeReference::new(
            Location::default(),
            None,
            AstName::ast_name_c_char(c"negate".as_ptr()),
            None,
            Location::default(),
            true,
            params,
        );

        allocator.alloc(reference) as *mut AstType
    }
}
