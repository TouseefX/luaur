use crate::functions::contains_subscripted_definition::contains_subscripted_definition;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::def::Def;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_binary(&mut self, b: *mut AstExprBinary) -> DataFlowResult {
        unsafe {
            let left = self.visit_expr_ast_expr((*b).left);
            let right = self.visit_expr_ast_expr((*b).right);

            let subscripted = ((*b).op == AstExprBinary_Op::And || (*b).op == AstExprBinary_Op::Or)
                && (contains_subscripted_definition(left.def as *const Def)
                    || contains_subscripted_definition(right.def as *const Def));

            let def = (*self.def_arena).fresh_cell(
                Symbol::default(),
                (*b).base.base.location,
                subscripted,
            );

            DataFlowResult {
                def: def as *const core::ffi::c_void,
                parent: core::ptr::null(),
            }
        }
    }
}
