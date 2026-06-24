use crate::type_aliases::def_id_control_flow_graph::DefId;
use luaur_ast::records::ast_stat_local::AstStatLocal;

#[derive(Debug, Clone)]
pub struct Declare {
    pub def: DefId,
    pub source: *mut AstStatLocal,
}

#[allow(non_snake_case)]
impl Declare {
    pub fn declare(def: DefId, source: *mut AstStatLocal) -> Self {
        Self { def, source }
    }
}
