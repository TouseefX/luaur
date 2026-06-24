use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
use luaur_analysis::records::phi::Phi;
use luaur_analysis::type_aliases::def_id_def::DefId;

impl DataFlowGraphFixture {
    pub fn check_operands(&self, phi: *const Phi, operands: Vec<DefId>) {
        unsafe {
            let phi_ref = &*phi;
            let mut operand_set: Vec<DefId> = Vec::new();
            for o in operands {
                operand_set.push(o);
            }
            assert_eq!(phi_ref.operands.len(), operand_set.len());
            for o in &phi_ref.operands {
                assert!(operand_set.contains(o));
            }
        }
    }
}
