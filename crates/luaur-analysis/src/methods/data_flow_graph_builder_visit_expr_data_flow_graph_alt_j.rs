use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr_unary::AstExprUnary;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_unary(&mut self, u: *mut AstExprUnary) -> DataFlowResult {
        unsafe {
            self.visit_expr_ast_expr((*u).expr);

            let def =
                (*self.def_arena).fresh_cell(Symbol::default(), (*u).base.base.location, false);
            DataFlowResult {
                def: def as *const core::ffi::c_void,
                parent: core::ptr::null(),
            }
        }
    }
}
