use crate::records::dcr_logger::DcrLogger;
use crate::type_aliases::type_pack_id::TypePackId;

impl DcrLogger {
    pub fn pop_block_type_pack_id(&mut self, block: TypePackId) {
        let _ = block;
        self.pop_block_type_id(unsafe { core::mem::transmute(block) });
    }
}
