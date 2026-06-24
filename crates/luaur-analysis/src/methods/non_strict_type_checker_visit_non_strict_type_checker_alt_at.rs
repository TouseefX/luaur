use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_error::AstTypeError;
use luaur_ast::records::ast_type_function::AstTypeFunction;
use luaur_ast::records::ast_type_group::AstTypeGroup;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;
use luaur_ast::records::ast_type_optional::AstTypeOptional;
use luaur_ast::records::ast_type_reference::AstTypeReference;
use luaur_ast::records::ast_type_singleton_bool::AstTypeSingletonBool;
use luaur_ast::records::ast_type_singleton_string::AstTypeSingletonString;
use luaur_ast::records::ast_type_table::AstTypeTable;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;
use luaur_ast::records::ast_type_union::AstTypeUnion;
use luaur_common::LUAU_ASSERT;

impl NonStrictTypeChecker {
    pub fn visit_ast_type(&mut self, ty: *mut AstType) {
        unsafe {
            if ty.is_null() {
                return;
            }

            let node = ty as *mut AstNode;
            if (*node).is::<AstTypeReference>() {
                self.visit_ast_type_reference(node as *mut AstTypeReference);
            } else if (*node).is::<AstTypeTable>() {
                self.visit_ast_type_table(node as *mut AstTypeTable);
            } else if (*node).is::<AstTypeFunction>() {
                self.visit_ast_type_function(node as *mut AstTypeFunction);
            } else if (*node).is::<AstTypeTypeof>() {
                self.visit_ast_type_typeof(node as *mut AstTypeTypeof);
            } else if (*node).is::<AstTypeUnion>() {
                self.visit_ast_type_union(node as *mut AstTypeUnion);
            } else if (*node).is::<AstTypeIntersection>() {
                self.visit_ast_type_intersection(node as *mut AstTypeIntersection);
            } else if (*node).is::<AstTypeGroup>() {
                let group = node as *mut AstTypeGroup;
                self.visit_ast_type((*group).type_);
            } else if (*node).is::<AstTypeOptional>() {
                let optional = node as *mut AstTypeOptional;
                self.visit_ast_type((*optional).type_);
            } else if (*node).is::<AstTypeError>() {
                let error = node as *mut AstTypeError;
                for i in 0..(*error).types.size {
                    self.visit_ast_type(*(*error).types.data.add(i));
                }
            } else if (*node).is::<AstTypeSingletonBool>() || (*node).is::<AstTypeSingletonString>()
            {
                return;
            } else {
                LUAU_ASSERT!(false);
            }
        }
    }
}
