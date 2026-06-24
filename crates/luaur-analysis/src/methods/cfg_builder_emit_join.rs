use crate::methods::block_set_reaching_definition::block_set_reaching_definition;
use crate::records::block::Block;
use crate::records::cfg_builder::CfgBuilder;
use crate::records::join::Join;
use crate::records::symbol::Symbol;

impl CfgBuilder {
    /// `Join* CFGBuilder::emitJoin(Block* block, Symbol sym)`.
    /// Reference: `ControlFlowGraph.cpp:258-265`.
    pub fn emit_join(&mut self, block: *mut Block, sym: Symbol) -> *mut Join {
        let def = self.new_definition(sym.clone());
        let j: *mut Join = self.emit::<Join, _>(block, def);
        let block_ref = unsafe { &mut *block };
        block_set_reaching_definition(block_ref, sym, def);
        self.incomplete_joins.get_or_insert(block).insert(j);
        j
    }
}
