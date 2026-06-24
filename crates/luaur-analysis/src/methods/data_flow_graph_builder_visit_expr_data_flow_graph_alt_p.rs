use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr_error::AstExprError;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_error(&mut self, error: *mut AstExprError) -> DataFlowResult {
        unsafe {
            {
                let unreachable: *mut DfgScope =
                    self.make_child_scope(crate::enums::scope_type::ScopeType::Linear);
                let _ps = PushScope::push_scope(&mut self.scope_stack, unreachable);

                for expr in (*error).expressions.iter() {
                    self.visit_expr_ast_expr(*expr);
                }
            }

            let def =
                (*self.def_arena).fresh_cell(Symbol::default(), (*error).base.base.location, false);
            DataFlowResult {
                def: def as *const core::ffi::c_void,
                parent: core::ptr::null(),
            }
        }
    }
}
