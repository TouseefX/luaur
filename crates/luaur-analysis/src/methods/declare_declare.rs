use crate::records::declare::Declare;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl Declare {
    pub fn declare_declare(&mut self, def: DefId, source: *mut AstStatLocal) {
        self.def = def;
        self.source = source;
    }
}
