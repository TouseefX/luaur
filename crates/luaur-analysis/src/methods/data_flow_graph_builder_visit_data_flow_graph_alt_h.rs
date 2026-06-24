use crate::enums::control_flow::ControlFlow;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use luaur_ast::records::ast_stat_return::AstStatReturn;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_return(&mut self, r: *mut AstStatReturn) -> ControlFlow {
        let r = r;

        unsafe {
            let list = (*r).list;
            for i in 0..list.size {
                let e = *list.data.add(i);
                let _ = self.visit_expr_ast_expr(e);
            }
        }

        ControlFlow::Returns
    }
}
