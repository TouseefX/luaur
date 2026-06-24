use crate::records::block::Block;
use crate::records::symbol::Symbol;
use crate::type_aliases::definition::Definition;

pub fn block_get_reaching_definition(block: &Block, sym: Symbol) -> *mut Definition {
    if let Some(v) = block.reaching_definitions.find(&sym) {
        *v
    } else {
        core::ptr::null_mut()
    }
}
