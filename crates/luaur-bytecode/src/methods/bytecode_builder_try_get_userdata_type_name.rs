use crate::records::bytecode_builder::BytecodeBuilder;
use luaur_common::enums::luau_bytecode_type::{
    LuauBytecodeType, LBC_TYPE_OPTIONAL_BIT, LBC_TYPE_TAGGED_USERDATA_BASE,
};

impl BytecodeBuilder {
    pub fn try_get_userdata_type_name(&self, type_: LuauBytecodeType) -> *const core::ffi::c_char {
        // C++ `unsigned((type & ~LBC_TYPE_OPTIONAL_BIT) - LBC_TYPE_TAGGED_USERDATA_BASE)`: the
        // subtraction is done in (signed) int and cast to unsigned, so a non-userdata type wraps
        // to a huge index that fails the bounds check. The u16 subtraction here underflow-panicked.
        let index = ((type_.0 & !(LBC_TYPE_OPTIONAL_BIT.0)) as i32
            - LBC_TYPE_TAGGED_USERDATA_BASE.0 as i32) as u32;

        if index < self.userdata_types.len() as u32 {
            let userdata_type = &self.userdata_types[index as usize];
            userdata_type.name.as_ptr() as *const core::ffi::c_char
        } else {
            core::ptr::null()
        }
    }
}
