use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_if_else(&mut self, i: *mut AstExprIfElse) -> DataFlowResult {
        unsafe {
            self.visit_expr_ast_expr((*i).condition);
            self.visit_expr_ast_expr((*i).true_expr);
            self.visit_expr_ast_expr((*i).false_expr);

            let def =
                (*self.def_arena).fresh_cell(Symbol::default(), (*i).base.base.location, false);

            DataFlowResult {
                def: def as *const core::ffi::c_void,
                parent: core::ptr::null(),
            }
        }
    }
}
