use crate::records::ir_inst::IrInst;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VmExitStoreRecord {
    pub inst_idx: u32,
    pub backup: IrInst,
}

#[allow(non_upper_case_globals)]
impl VmExitStoreRecord {
    pub const instIdx: u32 = 0xffffffff;
}

impl Default for VmExitStoreRecord {
    fn default() -> Self {
        Self {
            inst_idx: 0xffffffff,
            backup: unsafe { core::mem::zeroed() },
        }
    }
}
