use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def::Def;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_error::AstExprError;

impl DataFlowGraphBuilder {
    pub fn visit_l_value_ast_expr_error_def_id(
        &mut self,
        error: *mut AstExprError,
        _incoming_def: DefId,
    ) -> DefId {
        self.visit_expr_ast_expr(error as *mut AstExpr).def as *const Def
    }
}
