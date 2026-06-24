use crate::functions::collect_operands::collect_operands;
use crate::functions::get_def::get_def_id;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::phi::Phi;
use crate::type_aliases::def_id_def::DefId;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl DataFlowGraphBuilder {
    pub fn resolve_captures(&mut self) {
        for (_symbol, capture) in self.captures.iter() {
            let mut operands: Vec<DefId> = Vec::new();
            for i in capture.version_offset..capture.all_versions.len() {
                collect_operands(capture.all_versions[i], &mut operands);
            }

            for capture_def in &capture.capture_defs {
                let phi_ptr = get_def_id::<Phi>(*capture_def) as *mut Phi;
                LUAU_ASSERT!(!phi_ptr.is_null());
                unsafe {
                    LUAU_ASSERT!((*phi_ptr).operands.is_empty());
                    (*phi_ptr).operands = operands.clone();
                }
            }
        }
    }
}
