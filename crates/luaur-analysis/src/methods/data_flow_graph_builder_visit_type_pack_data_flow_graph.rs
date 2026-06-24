use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::internal_error_reporter::InternalErrorReporter;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;
use luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic;
use luaur_common::LUAU_ASSERT;

impl DataFlowGraphBuilder {
    pub fn visit_type_pack_ast_type_pack(&mut self, p: *mut AstTypePack) {
        unsafe {
            let node = p as *mut AstNode;
            if (*node).is::<AstTypePackExplicit>() {
                self.visit_type_pack_ast_type_pack_explicit(p as *mut AstTypePackExplicit);
            } else if (*node).is::<AstTypePackVariadic>() {
                self.visit_type_pack_ast_type_pack_variadic(p as *mut AstTypePackVariadic);
            } else if (*node).is::<AstTypePackGeneric>() {
                // ok
            } else {
                LUAU_ASSERT!(false);
            }
        }
    }
}
