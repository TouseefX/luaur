use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::const_prop_state::ConstPropState;
use crate::records::register_info::RegisterInfo;

impl ConstPropState {
    pub fn invalidate_table_array_size_register_info(&mut self, reg: &mut RegisterInfo) {
        CODEGEN_ASSERT!(!luaur_common::FFlag::LuauCodegenExtraTableOpts.get());
        reg.known_table_array_size_deprecated = -1;
    }
}
