//! @interface-stub
use crate::records::def::Def;
use crate::records::def_arena::DefArena;
use crate::records::refinement_key::RefinementKey;
use crate::records::refinement_key_arena::RefinementKeyArena;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction;
use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct DataFlowGraph {
    pub(crate) def_arena: *mut DefArena,
    pub(crate) key_arena: *mut RefinementKeyArena,
    pub(crate) ast_defs: DenseHashMap<*const AstExpr, *const Def>,
    pub(crate) local_defs: DenseHashMap<*const AstLocal, *const Def>,
    pub(crate) declared_defs: DenseHashMap<*const AstStat, *const Def>,
    pub(crate) def_to_symbol: DenseHashMap<*const Def, Symbol>,
    pub(crate) ast_refinement_keys: DenseHashMap<*const AstExpr, *const RefinementKey>,
}

impl DataFlowGraph {
    /// C++ `DataFlowGraph(DataFlowGraph&&) = default;` move-ctor; the Rust port
    /// moves by value, so this special member has no call site.
    pub fn data_flow_graph_data_flow_graph_mut(&mut self) {
        unreachable!("C++ DataFlowGraph move-ctor; Rust moves by value — no call site")
    }
    /// C++ `DataFlowGraph(const DataFlowGraph&) = delete;` — the deleted copy
    /// ctor (DataFlowGraph is non-copyable); never callable in C++ either.
    pub fn data_flow_graph_data_flow_graph(&self) {
        unreachable!("C++ DataFlowGraph copy-ctor is `= delete` — non-copyable, no call site")
    }
    /// Skeleton artifact of the private arena ctor with a malformed `&self`/no-arg
    /// signature; the real ctor is `DataFlowGraph::data_flow_graph(def_arena, key_arena)`
    /// (`methods/data_flow_graph_data_flow_graph_data_flow_graph_alt_c.rs`).
    pub fn data_flow_graph_data_flow_graph_not_null_def_arena_not_null_refinement_key_arena(&self) {
        unreachable!("superseded by DataFlowGraph::data_flow_graph(def_arena, key_arena) — no call site")
    }
    /// `DefId DataFlowGraph::getDef(const AstExpr* expr) const`.
    /// Reference: `DataFlowGraph.cpp` — `getDefOptional` plus an assert.
    pub fn get_def_ast_expr(&self, expr: *const AstExpr) -> DefId {
        let def = self.ast_defs.find(&expr);
        LUAU_ASSERT!(def.is_some());
        *def.unwrap()
    }

    /// `DefId DataFlowGraph::getDef(const AstLocal* local) const`. Reference: `DataFlowGraph.cpp:79-84`.
    pub fn get_def_ast_local(&self, local: *const AstLocal) -> DefId {
        let def = self.local_defs.find(&local);
        LUAU_ASSERT!(def.is_some());
        *def.unwrap()
    }

    /// `DefId DataFlowGraph::getDef(const AstStatDeclareGlobal* global) const`. Reference: `DataFlowGraph.cpp:86-91`.
    pub fn get_def_ast_stat_declare_global(&self, global: *const AstStatDeclareGlobal) -> DefId {
        let def = self.declared_defs.find(&(global as *const AstStat));
        LUAU_ASSERT!(def.is_some());
        *def.unwrap()
    }

    /// `DefId DataFlowGraph::getDef(const AstStatDeclareFunction* func) const`. Reference: `DataFlowGraph.cpp:93-98`.
    pub fn get_def_ast_stat_declare_function(&self, func: *const AstStatDeclareFunction) -> DefId {
        let def = self.declared_defs.find(&(func as *const AstStat));
        LUAU_ASSERT!(def.is_some());
        *def.unwrap()
    }
    /// Skeleton artifact duplicating the private arena ctor; the real
    /// constructor (`DataFlowGraph::data_flow_graph(def_arena, key_arena)`,
    /// `methods/data_flow_graph_data_flow_graph_data_flow_graph_alt_c.rs`) returns
    /// `Self`, whereas this generated variant takes `&self` and returns `()`.
    pub fn data_flow_graph_not_null_def_arena_not_null_refinement_key_arena(
        &self,
        _def_arena: *mut DefArena,
        _key_arena: *mut RefinementKeyArena,
    ) {
        unreachable!("superseded by DataFlowGraph::data_flow_graph(def_arena, key_arena) — no call site")
    }
}
