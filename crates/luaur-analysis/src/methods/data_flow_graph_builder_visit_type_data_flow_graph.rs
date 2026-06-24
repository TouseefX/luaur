use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
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

impl DataFlowGraphBuilder {
    pub fn visit_type_ast_type(&mut self, t: *mut AstType) {
        unsafe {
            let node = t as *mut luaur_ast::records::ast_node::AstNode;
            if (*node).is::<AstTypeReference>() {
                self.visit_type_ast_type_reference(node as *mut AstTypeReference);
            } else if (*node).is::<AstTypeTable>() {
                self.visit_type_ast_type_table(node as *mut AstTypeTable);
            } else if (*node).is::<AstTypeFunction>() {
                self.visit_type_ast_type_function(node as *mut AstTypeFunction);
            } else if (*node).is::<AstTypeTypeof>() {
                self.visit_type_ast_type_typeof(node as *mut AstTypeTypeof);
            } else if (*node).is::<AstTypeOptional>() {
                return;
            } else if (*node).is::<AstTypeUnion>() {
                self.visit_type_ast_type_union(node as *mut AstTypeUnion);
            } else if (*node).is::<AstTypeIntersection>() {
                self.visit_type_ast_type_intersection(node as *mut AstTypeIntersection);
            } else if (*node).is::<AstTypeError>() {
                self.visit_type_ast_type_error(node as *mut AstTypeError);
            } else if (*node).is::<AstTypeSingletonBool>() {
                return;
            } else if (*node).is::<AstTypeSingletonString>() {
                return;
            } else if (*node).is::<AstTypeGroup>() {
                let group = node as *mut AstTypeGroup;
                self.visit_type_ast_type((*group).type_);
            } else {
                LUAU_ASSERT!(false);
            }
        }
    }
}
