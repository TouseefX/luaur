use crate::enums::block_kind::BlockKind;
use crate::records::block::Block;
use alloc::string::String;

impl Block {
    pub fn block_block(&mut self, kind: BlockKind, debug_name: String) {
        self.kind = kind;
        self.debug_name = debug_name;
    }
}
