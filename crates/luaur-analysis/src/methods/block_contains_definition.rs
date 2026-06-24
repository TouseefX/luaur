use crate::records::block::Block;
use crate::records::symbol::Symbol;

pub fn block_contains_definition(block: &Block, sym: Symbol) -> bool {
    block.reaching_definitions.contains(&sym)
}
