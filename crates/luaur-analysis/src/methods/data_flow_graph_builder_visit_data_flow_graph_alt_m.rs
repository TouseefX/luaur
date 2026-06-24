use crate::enums::control_flow::ControlFlow;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def::Def;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_assign(&mut self, a: *mut AstStatAssign) -> ControlFlow {
        unsafe {
            let mut defs: alloc::vec::Vec<DefId> = alloc::vec::Vec::with_capacity((*a).values.size);

            for expr in (*a).values.iter() {
                defs.push(self.visit_expr_ast_expr(*expr).def as *const Def);
            }

            for (i, var) in (*a).vars.iter().enumerate() {
                let var = *var;
                let incoming_def = if i < defs.len() {
                    defs[i]
                } else {
                    (*self.def_arena).fresh_cell(Symbol::default(), (*var).base.location, false)
                };
                self.visit_l_value_ast_expr_def_id(var, incoming_def);
            }
        }

        ControlFlow::None
    }
}
