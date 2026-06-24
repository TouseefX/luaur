use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
use luaur_analysis::records::data_flow_graph::DataFlowGraph;
use luaur_analysis::records::def_arena::DefArena;
use luaur_analysis::records::internal_error_reporter::InternalErrorReporter;
use luaur_analysis::records::refinement_key_arena::RefinementKeyArena;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_common::FFlag;

#[derive(Debug)]
#[repr(C)]
pub struct DataFlowGraphFixture {
    pub dcr: ScopedFastFlag,
    pub def_arena: DefArena,
    pub key_arena: RefinementKeyArena,
    pub handle: InternalErrorReporter,
    pub allocator: Allocator,
    pub names: AstNameTable,
    pub module: *mut AstStatBlock,
    pub graph: Option<DataFlowGraph>,
}

impl Default for DataFlowGraphFixture {
    fn default() -> Self {
        let mut allocator = Allocator::allocator();
        let names = AstNameTable::new(&mut allocator);

        Self {
            dcr: ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
            def_arena: DefArena::default(),
            key_arena: RefinementKeyArena::default(),
            handle: InternalErrorReporter::default(),
            allocator,
            names,
            module: core::ptr::null_mut(),
            graph: None,
        }
    }
}

impl DataFlowGraphFixture {
    pub fn new() -> Self {
        Self::default()
    }
}
