use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;

impl ConstPropState {
    pub fn const_prop_state_const_prop_state(
        build: &mut IrBuilder,
        function: &mut IrFunction,
    ) -> Self {
        Self {
            build: build as *mut IrBuilder,
            function: function as *mut IrFunction,
            regs: [crate::records::register_info::RegisterInfo::default(); 256],
            max_reg: 0,
            inst_pos: 0,
            in_safe_env: false,
            checked_gc: false,
            inst_link: luaur_common::records::dense_hash_map::DenseHashMap::new(k_invalid_inst_idx),
            inst_tag: luaur_common::records::dense_hash_map::DenseHashMap::new(k_invalid_inst_idx),
            inst_value: luaur_common::records::dense_hash_map::DenseHashMap::new(k_invalid_inst_idx),
            value_map: luaur_common::records::dense_hash_map::DenseHashMap::new(IrInst::default()),
            upvalue_map: luaur_common::records::dense_hash_map::DenseHashMap::new(0xff),
            hash_value_cache: luaur_common::records::dense_hash_map::DenseHashMap::new(
                k_invalid_inst_idx,
            ),
            array_value_cache: alloc::vec::Vec::new(),
            try_num_to_index_cache: alloc::vec::Vec::new(),
            get_slot_node_cache: alloc::vec::Vec::new(),
            check_slot_match_cache: alloc::vec::Vec::new(),
            get_arr_addr_cache: alloc::vec::Vec::new(),
            check_array_size_cache: alloc::vec::Vec::new(),
            check_buffer_len_cache: alloc::vec::Vec::new(),
            useradata_tag_cache: alloc::vec::Vec::new(),
            buffer_load_store_info: alloc::vec::Vec::new(),
            load_env_idx: k_invalid_inst_idx,
            inst_not_readonly: luaur_common::records::dense_hash_set::DenseHashSet::new(
                k_invalid_inst_idx,
            ),
            inst_no_metatable: luaur_common::records::dense_hash_set::DenseHashSet::new(
                k_invalid_inst_idx,
            ),
            inst_array_size: luaur_common::records::dense_hash_map::DenseHashMap::new(
                k_invalid_inst_idx,
            ),
            range_end_temp: alloc::vec::Vec::new(),
        }
    }
}
