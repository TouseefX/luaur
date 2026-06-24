use crate::functions::nth::nth_T;
use crate::functions::query::query;
use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
use luaur_analysis::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl DataFlowGraphFixture {
    pub fn get_local_def(&self, stat_n: i32, var_index: usize) -> DefId {
        let local_stat = query::<AstStatLocal>(
            self.module as *mut AstNode,
            vec![nth_T::<AstStatLocal>(stat_n)],
        );
        assert!(!local_stat.is_null());

        let local = unsafe { *(*local_stat).vars.data.add(var_index) };
        unsafe {
            self.graph
                .as_ref()
                .expect("DataFlowGraphFixture.graph is None")
                .get_def_ast_local(local)
        }
    }
}
