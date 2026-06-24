use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def::Def;
use crate::records::dfg_scope::DfgScope;
use crate::type_aliases::def_id_def::DefId;
use alloc::collections::BTreeMap;
use alloc::string::String;

impl DataFlowGraphBuilder {
    /// `void DataFlowGraphBuilder::joinProps(DfgScope* result, const DfgScope& a, const DfgScope& b)`.
    /// Reference: `DataFlowGraph.cpp:246-294`.
    pub fn join_props(&mut self, result: *mut DfgScope, a: &DfgScope, b: &DfgScope) {
        let def_arena = self.def_arena;
        // C++ lambda `phinodify`: merges per-key defs of `a`/`b` into `scope->props[parent]`.
        let phinodify = |scope: *mut DfgScope,
                         a_props: &BTreeMap<String, *const Def>,
                         b_props: &BTreeMap<String, *const Def>,
                         parent: DefId| unsafe {
            let p: &mut BTreeMap<String, *const Def> = (*scope).props.get_or_insert(parent);
            for (k, def_a) in a_props.iter() {
                if let Some(it) = b_props.get(k) {
                    p.insert(k.clone(), (*def_arena).phi_def_id_def_id(*it, *def_a));
                } else if let Some(it) = p.get(k).copied() {
                    p.insert(k.clone(), (*def_arena).phi_def_id_def_id(it, *def_a));
                } else if let Some(def2) = (*scope).lookup_def_id_string(parent, k) {
                    p.insert(k.clone(), (*def_arena).phi_def_id_def_id(def2, *def_a));
                } else {
                    p.insert(k.clone(), *def_a);
                }
            }

            for (k, def_b) in b_props.iter() {
                if a_props.get(k).is_some() {
                    continue;
                } else if let Some(it) = p.get(k).copied() {
                    p.insert(k.clone(), (*def_arena).phi_def_id_def_id(it, *def_b));
                } else if let Some(def2) = (*scope).lookup_def_id_string(parent, k) {
                    p.insert(k.clone(), (*def_arena).phi_def_id_def_id(def2, *def_b));
                } else {
                    p.insert(k.clone(), *def_b);
                }
            }
        };

        unsafe {
            for (def, a1) in a.props.iter() {
                (*result).props.try_insert(*def, BTreeMap::new());
                if let Some(a2) = b.props.find(def) {
                    let a2 = a2.clone();
                    phinodify(result, a1, &a2, *def);
                } else if let Some(a2) = (*result).props.find(def) {
                    let a2 = a2.clone();
                    phinodify(result, a1, &a2, *def);
                }
            }

            for (def, a1) in b.props.iter() {
                (*result).props.try_insert(*def, BTreeMap::new());
                if a.props.find(def).is_some() {
                    continue;
                } else if let Some(a2) = (*result).props.find(def) {
                    let a2 = a2.clone();
                    phinodify(result, a1, &a2, *def);
                }
            }
        }
    }
}
