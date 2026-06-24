use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_interp_string(
        &mut self,
        i: *mut AstExprInterpString,
    ) -> DataFlowResult {
        unsafe {
            for expr in (*i).expressions.iter() {
                self.visit_expr_ast_expr(*expr);
            }

            let def =
                (*self.def_arena).fresh_cell(Symbol::default(), (*i).base.base.location, false);
            DataFlowResult {
                def: def as *const core::ffi::c_void,
                parent: core::ptr::null(),
            }
        }
    }
}
