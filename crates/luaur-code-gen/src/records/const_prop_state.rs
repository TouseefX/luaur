use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::array_value_entry::ArrayValueEntry;
use crate::records::buffer_access_base::BufferAccessBase;
use crate::records::buffer_load_store_info::BufferLoadStoreInfo;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_inst_eq::IrInstEq;
use crate::records::ir_inst_hash::IrInstHash;
use crate::records::ir_op::IrOp;
use crate::records::node_slot_state::NodeSlotState;
use crate::records::numbered_instruction::NumberedInstruction;
use crate::records::register_info::RegisterInfo;
use crate::records::register_link::RegisterLink;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug)]
pub struct ConstPropState {
    pub build: *mut IrBuilder,
    pub function: *mut IrFunction,
    pub regs: [RegisterInfo; 256],
    pub max_reg: i32,
    pub inst_pos: u32,
    pub in_safe_env: bool,
    pub checked_gc: bool,
    pub inst_link: DenseHashMap<u32, RegisterLink>,
    pub inst_tag: DenseHashMap<u32, u8>,
    pub inst_value: DenseHashMap<u32, IrOp>,
    pub value_map: DenseHashMap<IrInst, u32, IrInstHash, IrInstEq>,
    pub upvalue_map: DenseHashMap<u8, u32>,
    pub hash_value_cache: DenseHashMap<u32, u32>,
    pub array_value_cache: Vec<ArrayValueEntry>,
    pub try_num_to_index_cache: Vec<u32>,
    pub get_slot_node_cache: Vec<NumberedInstruction>,
    pub check_slot_match_cache: Vec<NodeSlotState>,
    pub get_arr_addr_cache: Vec<u32>,
    pub check_array_size_cache: Vec<u32>,
    pub check_buffer_len_cache: Vec<u32>,
    pub useradata_tag_cache: Vec<u32>,
    pub buffer_load_store_info: Vec<BufferLoadStoreInfo>,
    pub load_env_idx: u32,
    pub inst_not_readonly: DenseHashSet<u32>,
    pub inst_no_metatable: DenseHashSet<u32>,
    pub inst_array_size: DenseHashMap<u32, i32>,
    pub range_end_temp: Vec<u32>,
}

impl ConstPropState {
    pub fn clear(&mut self) {
        for i in 0..=self.max_reg as usize {
            self.regs[i] = RegisterInfo::default();
        }
        self.max_reg = 0;
        self.inst_pos = 0;
        self.in_safe_env = false;
        self.checked_gc = false;
        self.inst_link.clear();
        self.inst_tag.clear();
        self.inst_value.clear();
        self.value_map.clear();
        self.upvalue_map.clear();
        self.hash_value_cache.clear();
        self.array_value_cache.clear();
        self.try_num_to_index_cache.clear();
        self.get_slot_node_cache.clear();
        self.check_slot_match_cache.clear();
        self.get_arr_addr_cache.clear();
        self.check_array_size_cache.clear();
        self.check_buffer_len_cache.clear();
        self.useradata_tag_cache.clear();
        self.buffer_load_store_info.clear();
        self.load_env_idx = !0;
        self.inst_not_readonly.clear();
        self.inst_no_metatable.clear();
        self.inst_array_size.clear();
        self.range_end_temp.clear();
    }
}
