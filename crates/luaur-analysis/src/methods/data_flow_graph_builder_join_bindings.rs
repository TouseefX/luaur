use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;

impl DataFlowGraphBuilder {
    pub fn join_bindings(&mut self, p: *mut DfgScope, a: &DfgScope, b: &DfgScope) {
        unsafe {
            for (sym, def1) in a.bindings.iter() {
                if let Some(def2) = b.bindings.find(sym) {
                    let phi = (*self.def_arena).phi_def_id_def_id(*def1, *def2);
                    *(*p).bindings.get_or_insert(sym.clone()) = phi;
                } else if let Some(def2) = (*p).lookup_symbol(sym.clone()) {
                    let phi = (*self.def_arena).phi_def_id_def_id(*def1, def2);
                    *(*p).bindings.get_or_insert(sym.clone()) = phi;
                }
            }

            for (sym, def1) in b.bindings.iter() {
                if let Some(def2) = (*p).lookup_symbol(sym.clone()) {
                    let phi = (*self.def_arena).phi_def_id_def_id(*def1, def2);
                    *(*p).bindings.get_or_insert(sym.clone()) = phi;
                }
            }
        }
    }
}
