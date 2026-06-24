extern crate alloc;

use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::bytecode_block::BytecodeBlock;
use crate::records::bytecode_mapping::BytecodeMapping;
use crate::records::bytecode_type_info::BytecodeTypeInfo;
use crate::records::bytecode_types::BytecodeTypes;
use crate::records::cfg_info::CfgInfo;
use crate::records::ir_block::IrBlock;
use crate::records::ir_const::IrConst;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::records::lowering_stats::LoweringStats;
use crate::records::store_location_hint::StoreLocationHint;
use crate::records::value_restore_location::ValueRestoreLocation;
use crate::records::vm_exit_sync_info::VmExitSyncInfo;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
#[repr(C)]
pub struct IrFunction {
    pub blocks: Vec<IrBlock>,
    pub instructions: Vec<IrInst>,
    pub constants: Vec<IrConst>,

    pub bc_blocks: Vec<BytecodeBlock>,
    pub bc_types: Vec<BytecodeTypes>,

    pub bc_mapping: Vec<BytecodeMapping>,
    pub entry_block: u32,
    pub entry_location: u32,
    pub end_location: u32,

    pub extra_native_data: Vec<u32>,

    pub value_restore_ops: Vec<ValueRestoreLocation>,
    pub valid_restore_op_blocks: Vec<u32>,
    pub store_location_hints: DenseHashMap<u32, StoreLocationHint>,

    pub vm_exit_info: DenseHashMap<u32, VmExitSyncInfo>,
    pub block_to_vm_exit_map: DenseHashMap<u32, u32>,

    pub bc_original_type_info: BytecodeTypeInfo,
    pub bc_type_info: BytecodeTypeInfo,

    pub proto: *mut luaur_vm::records::proto::Proto,
    pub variadic: bool,

    pub cfg: CfgInfo,

    pub stats: *mut LoweringStats,

    pub record_counters: bool,

    pub jit_rng_state: u64,

    pub block_exit_tags: Vec<Vec<u8>>,
}

impl luaur_common::records::dense_hash_table::DenseDefault for StoreLocationHint {
    fn dense_default() -> Self {
        Self {
            op: IrOp { kind_and_index: 0 },
            inst_idx: !0u32,
            kind: crate::enums::ir_value_kind::IrValueKind::None,
        }
    }
}

impl luaur_common::records::dense_hash_table::DenseDefault for VmExitSyncInfo {
    fn dense_default() -> Self {
        Self {
            reg_stores: Vec::new(),
            block: IrOp { kind_and_index: 0 },
            vm_exit: IrOp { kind_and_index: 0 },
            arg_ops: luaur_common::records::small_vector::SmallVector::new(),
        }
    }
}

impl Default for IrFunction {
    fn default() -> Self {
        Self {
            blocks: Vec::new(),
            instructions: Vec::new(),
            constants: Vec::new(),
            bc_blocks: Vec::new(),
            bc_types: Vec::new(),
            bc_mapping: Vec::new(),
            entry_block: 0,
            entry_location: 0,
            end_location: 0,
            extra_native_data: Vec::new(),
            value_restore_ops: Vec::new(),
            valid_restore_op_blocks: Vec::new(),
            // kInvalidInstIdx is ~0u32
            store_location_hints: DenseHashMap::new(!0u32),
            vm_exit_info: DenseHashMap::new(!0u32),
            block_to_vm_exit_map: DenseHashMap::new(!0u32),
            bc_original_type_info: BytecodeTypeInfo::default(),
            bc_type_info: BytecodeTypeInfo::default(),
            proto: core::ptr::null_mut(),
            variadic: false,
            cfg: CfgInfo::default(),
            stats: core::ptr::null_mut(),
            record_counters: false,
            jit_rng_state: 0,
            block_exit_tags: Vec::new(),
        }
    }
}
