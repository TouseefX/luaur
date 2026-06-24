use crate::records::assign::Assign;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl Assign {
    pub fn assign_assign(def: DefId, source: *mut AstStatAssign) -> Self {
        Self { def, source }
    }
}
