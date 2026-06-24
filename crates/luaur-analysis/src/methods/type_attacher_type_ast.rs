use crate::records::type_attacher::TypeAttacher;
use crate::records::type_rehydration_options::TypeRehydrationOptions;
use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::synthetic_names::SyntheticNames;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_type::AstType;

impl TypeAttacher {
    pub fn type_ast(&mut self, r#type: Option<TypeId>) -> *mut AstType {
        if r#type.is_none() {
            return core::ptr::null_mut();
        }

        // C++ `return Luau::visit(TypeRehydrationVisitor(allocator, &syntheticNames), (*type)->ty);`
        let ty = r#type.unwrap();
        let mut visitor = TypeRehydrationVisitor::type_rehydration_visitor_type_rehydration_visitor(
            self.allocator,
            &mut self.synthetic_names as *mut SyntheticNames,
            &TypeRehydrationOptions::default(),
        );
        visitor.visit_type(ty)
    }
}
