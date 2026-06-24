use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::records::union_type::UnionType;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_union::AstTypeUnion;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    pub fn operator_call_20(&mut self, uv: &UnionType) -> *mut AstType {
        let size = uv.options.len();
        let data = unsafe {
            (*self.allocator).allocate(core::mem::size_of::<*mut AstType>() * size)
                as *mut *mut AstType
        };

        for i in 0..size {
            // C++ `unionTypes.data[i] = Luau::visit(*this, uv.options[i]->ty);`
            let option_ty = uv.options[i];
            let rehydrated = self.visit_type(option_ty);
            unsafe { *data.add(i) = rehydrated };
        }

        let union_types = AstArray { data, size };

        let alloc = unsafe { &mut *self.allocator };
        alloc.alloc(AstTypeUnion::new(Location::default(), union_types)) as *mut AstType
    }
}
