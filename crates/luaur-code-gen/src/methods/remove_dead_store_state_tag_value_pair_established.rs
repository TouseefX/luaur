use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use crate::records::store_reg_info::StoreRegInfo;

impl RemoveDeadStoreState {
    pub fn tag_value_pair_established(&mut self, reg_info: &mut StoreRegInfo) -> bool {
        let tag_established = reg_info.tag_inst_idx != !0u32 || reg_info.known_tag != 0xff;
        let value_established = reg_info.value_inst_idx != !0u32
            || reg_info.known_tag == luaur_vm::enums::lua_type::lua_Type::LUA_TNIL as u8;
        tag_established && value_established
    }
}
