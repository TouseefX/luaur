use crate::records::cfg_fixture::CfgFixture;
use luaur_analysis::functions::find_node_at_position_ast_query_alt_b::find_node_at_position_ast_stat_block_position;
use luaur_analysis::records::control_flow_graph::ControlFlowGraph;
use luaur_analysis::type_aliases::definition::Definition;
use luaur_ast::records::position::Position;

impl CfgFixture {
    pub fn get_definition_at_pos(
        &mut self,
        cfg: &ControlFlowGraph,
        pos: Position,
    ) -> *mut Definition {
        assert!(!self.root.is_null());

        let node = find_node_at_position_ast_stat_block_position(unsafe { &*self.root }, pos);
        assert!(!node.is_null());

        let expr = unsafe { (*node).as_expr() };
        assert!(!expr.is_null());

        let def = cfg.use_defs.find(&expr);
        assert!(def.is_some());
        *def.unwrap()
    }
}
