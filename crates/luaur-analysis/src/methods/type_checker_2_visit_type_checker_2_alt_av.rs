//! Faithful port of `TypeChecker2::visit(AstType*)` (TypeChecker2.cpp:2778-2798).
use crate::functions::follow_type::follow_type_id;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_function::AstTypeFunction;
use luaur_ast::records::ast_type_group::AstTypeGroup;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::ast_type_table::AstTypeTable;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;
use luaur_ast::records::ast_type_union::AstTypeUnion;

impl TypeChecker2 {
    pub fn visit_ast_type(&mut self, ty: *mut AstType) {
        unsafe {
            let resolved_ty = (*self.module)
                .ast_resolved_types
                .find(&(ty as *const AstType))
                .copied();
            if let Some(resolved_ty) = resolved_ty {
                self.check_for_type_function_inhabitance(
                    follow_type_id(resolved_ty),
                    (*ty).base.location,
                );
            }

            let node = ty as *mut AstNode;
            if (*node).is::<AstTypeReference>() {
                self.visit_ast_type_reference(ty as *mut AstTypeReference);
            } else if (*node).is::<AstTypeTable>() {
                self.visit_ast_type_table(ty as *mut AstTypeTable);
            } else if (*node).is::<AstTypeFunction>() {
                self.visit_ast_type_function(ty as *mut AstTypeFunction);
            } else if (*node).is::<AstTypeTypeof>() {
                self.visit_ast_type_typeof(ty as *mut AstTypeTypeof);
            } else if (*node).is::<AstTypeUnion>() {
                self.visit_ast_type_union(ty as *mut AstTypeUnion);
            } else if (*node).is::<AstTypeIntersection>() {
                self.visit_ast_type_intersection(ty as *mut AstTypeIntersection);
            } else if (*node).is::<AstTypeGroup>() {
                let group = ty as *mut AstTypeGroup;
                self.visit_ast_type((*group).type_);
            }
        }
    }
}
