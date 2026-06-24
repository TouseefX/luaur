use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
use luaur_analysis::functions::get_def::get_def_id;
use luaur_analysis::records::phi::Phi;
use luaur_analysis::type_aliases::def_id_def::DefId;

impl DataFlowGraphFixture {
    pub fn get_phi(&self, def: DefId) -> *const Phi {
        get_def_id::<Phi>(def)
    }
}
