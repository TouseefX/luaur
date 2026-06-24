use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::records::type_attacher::TypeAttacher;
use crate::records::type_rehydration_options::TypeRehydrationOptions;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::synthetic_names::SyntheticNames;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_array::AstArray;
use luaur_ast::records::ast_type::AstType;

impl TypeAttacher {
    pub fn type_ast_pack(&mut self, r#type: TypePackId) -> AstArray<*mut AstType> {
        let (v, _tail) = flatten_type_pack_id(r#type);

        let size = v.len();
        let data = unsafe {
            (*self.allocator).allocate(size * core::mem::size_of::<*mut AstType>())
                as *mut *mut AstType
        };

        let mut index = 0;
        for item in v.iter() {
            // C++ `result.data[i] = Luau::visit(TypeRehydrationVisitor(allocator, &syntheticNames), v[i]->ty);`
            let mut rehydrator =
                TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
                    self.allocator,
                    &mut self.synthetic_names as *mut SyntheticNames,
                    &TypeRehydrationOptions::default(),
                );
            let ast_type = rehydrator.visit_type(*item);
            unsafe {
                *data.add(index) = ast_type;
            }
            index += 1;
        }

        AstArray { data, size }
    }
}
