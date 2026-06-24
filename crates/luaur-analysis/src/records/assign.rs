use crate::type_aliases::def_id_control_flow_graph::DefId;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

#[derive(Debug, Clone)]
pub struct Assign {
    pub def: DefId,
    pub source: *mut AstStatAssign,
}

#[allow(non_snake_case)]
impl Assign {
    pub fn assign(def: DefId, source: *mut AstStatAssign) -> Self {
        Self { def, source }
    }
}
