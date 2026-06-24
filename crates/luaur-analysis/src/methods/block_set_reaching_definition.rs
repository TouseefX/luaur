use crate::records::block::Block;
use crate::records::symbol::Symbol;
use crate::type_aliases::definition::Definition;

pub fn block_set_reaching_definition(block: &mut Block, sym: Symbol, def: *mut Definition) {
    *block.reaching_definitions.get_or_insert(sym) = def;
}
