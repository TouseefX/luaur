use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::def::Def;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_expr_local::AstExprLocal;

fn refinement_def_id(def: DefId) -> crate::type_aliases::def_id_refinement::DefId {
    core::ptr::NonNull::new(def as *mut *const Def).unwrap()
}

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_local(&mut self, l: *mut AstExprLocal) -> DataFlowResult {
        unsafe {
            let local = (*l).local;
            let def = self.lookup_symbol_location(Symbol::from_local(local), (*local).location);
            let key = (*self.key_arena).leaf(refinement_def_id(def));
            *self.graph.def_to_symbol.get_or_insert(def) = Symbol::from_local(local);

            DataFlowResult {
                def: def as *const core::ffi::c_void,
                parent: key,
            }
        }
    }
}
