use crate::records::vm_exit_store_record::VmExitStoreRecord;
use luaur_common::records::small_vector::SmallVector;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VmExitStoreInfo {
    pub reg: u8,
    pub stores: SmallVector<VmExitStoreRecord, 2>,
}

impl Default for VmExitStoreInfo {
    fn default() -> Self {
        Self {
            reg: 0,
            stores: SmallVector::new(),
        }
    }
}
