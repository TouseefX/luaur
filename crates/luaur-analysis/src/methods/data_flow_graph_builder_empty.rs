use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def::Def;
use crate::records::def_arena::DefArena;
use crate::records::refinement_key_arena::RefinementKeyArena;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_stat::AstStat;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl DataFlowGraphBuilder {
    pub fn empty(def_arena: *mut DefArena, key_arena: *mut RefinementKeyArena) -> DataFlowGraph {
        DataFlowGraph {
            def_arena,
            key_arena,
            ast_defs: DenseHashMap::new(core::ptr::null::<AstExpr>()),
            local_defs: DenseHashMap::new(core::ptr::null::<AstLocal>()),
            declared_defs: DenseHashMap::new(core::ptr::null::<AstStat>()),
            def_to_symbol: DenseHashMap::new(core::ptr::null::<Def>()),
            ast_refinement_keys: DenseHashMap::new(core::ptr::null::<AstExpr>()),
        }
    }
}
