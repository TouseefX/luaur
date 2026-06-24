const LBC_TYPE_NIL: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_NIL.0 as u8;
const LBC_TYPE_BOOLEAN: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_BOOLEAN.0 as u8;
const LBC_TYPE_NUMBER: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_NUMBER.0 as u8;
const LBC_TYPE_STRING: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_STRING.0 as u8;
const LBC_TYPE_TABLE: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_TABLE.0 as u8;
const LBC_TYPE_FUNCTION: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_FUNCTION.0 as u8;
const LBC_TYPE_THREAD: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_THREAD.0 as u8;
const LBC_TYPE_USERDATA: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_USERDATA.0 as u8;
const LBC_TYPE_VECTOR: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_VECTOR.0 as u8;
const LBC_TYPE_BUFFER: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_BUFFER.0 as u8;
const LBC_TYPE_ANY: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_ANY.0 as u8;
const LBC_TYPE_INTEGER: u8 = luaur_common::enums::luau_bytecode_type::LBC_TYPE_INTEGER.0 as u8;

const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = 64;
const LBC_TYPE_TAGGED_USERDATA_END: u8 =
    luaur_common::enums::luau_bytecode_type::LBC_TYPE_TAGGED_USERDATA_END.0 as u8;
const LBC_TYPE_OPTIONAL_BIT: u8 = 128;

pub fn get_bytecode_type_name(
    mut r#type: u8,
    userdata_types: *const *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    // Optional bit should be handled externally
    r#type &= !LBC_TYPE_OPTIONAL_BIT;

    if r#type >= LBC_TYPE_TAGGED_USERDATA_BASE && r#type < LBC_TYPE_TAGGED_USERDATA_END {
        if !userdata_types.is_null() {
            return unsafe {
                *userdata_types.add((r#type - LBC_TYPE_TAGGED_USERDATA_BASE) as usize)
            };
        }

        return c"userdata".as_ptr();
    }

    match r#type {
        LBC_TYPE_NIL => return c"nil".as_ptr(),
        LBC_TYPE_BOOLEAN => return c"boolean".as_ptr(),
        LBC_TYPE_NUMBER => return c"number".as_ptr(),
        LBC_TYPE_INTEGER => return c"integer".as_ptr(),
        LBC_TYPE_STRING => return c"string".as_ptr(),
        LBC_TYPE_TABLE => return c"table".as_ptr(),
        LBC_TYPE_FUNCTION => return c"function".as_ptr(),
        LBC_TYPE_THREAD => return c"thread".as_ptr(),
        LBC_TYPE_USERDATA => return c"userdata".as_ptr(),
        LBC_TYPE_VECTOR => return c"vector".as_ptr(),
        LBC_TYPE_BUFFER => return c"buffer".as_ptr(),
        LBC_TYPE_ANY => return c"any".as_ptr(),
        _ => {}
    }

    luaur_common::LUAU_ASSERT!(false);
    core::ptr::null()
}
