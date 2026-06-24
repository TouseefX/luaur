use crate::functions::nth::AstNodeClass;
use crate::functions::query::query;
use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
use crate::records::nth::Nth;
use luaur_analysis::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_expr::AstExpr;

impl DataFlowGraphFixture {
    pub fn get_def<T: AstNodeClass>(&mut self, nths: Vec<Nth>) -> DefId {
        let node = query::<T>(
            self.module as *mut luaur_ast::records::ast_node::AstNode,
            nths,
        );
        luaur_common::LUAU_ASSERT!(!node.is_null());
        unsafe {
            (*self
                .graph
                .as_ref()
                .expect("DataFlowGraphFixture.graph is None"))
            .get_def(node as *const AstExpr)
        }
    }
}
