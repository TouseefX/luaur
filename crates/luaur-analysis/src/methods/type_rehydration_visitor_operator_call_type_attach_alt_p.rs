use crate::records::intersection_type::IntersectionType;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;
use luaur_ast::records::location::Location;

impl TypeRehydrationVisitor {
    pub fn operator_call_8(&mut self, uv: &IntersectionType) -> *mut AstType {
        let size = uv.parts.len();
        let data = unsafe {
            (*self.allocator).allocate(core::mem::size_of::<*mut AstType>() * size)
                as *mut *mut AstType
        };

        for i in 0..size {
            // C++ `intersectionTypes.data[i] = Luau::visit(*this, uv.parts[i]->ty);`
            let part_ty = uv.parts[i];
            let ast_part = self.visit_type(part_ty);
            unsafe { *data.add(i) = ast_part };
        }

        let intersection_types = AstArray { data, size };

        let location = Location::default();
        let alloc = unsafe { &mut *self.allocator };
        alloc.alloc(AstTypeIntersection::new(location, intersection_types)) as *mut AstType
    }
}
