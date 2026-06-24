extern crate alloc;

use crate::records::ir_op::IrOp;
use crate::records::vm_exit_store_info::VmExitStoreInfo;
use alloc::vec::Vec;
use luaur_common::records::small_vector::SmallVector;

#[derive(Debug, Clone)]
pub struct VmExitSyncInfo {
    pub reg_stores: Vec<VmExitStoreInfo>,
    pub block: IrOp,
    pub vm_exit: IrOp,
    pub arg_ops: SmallVector<IrOp, 2>,
}
