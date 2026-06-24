use luaur_vm::macros::getstr::getstr;

macro_rules! CODEGEN_ASSERT {
    ($expr:expr) => {
        assert!($expr);
    };
}

pub unsafe fn try_find_upvalue_name(
    proto: *const luaur_vm::records::proto::Proto,
    upval: i32,
) -> *const core::ffi::c_char {
    if !(*proto).upvalues.is_null() {
        CODEGEN_ASSERT!(upval < (*proto).sizeupvalues as i32);

        let upvalue = *(*proto).upvalues.add(upval as usize);
        if !upvalue.is_null() {
            return getstr(upvalue as *const _);
        }
    }

    core::ptr::null()
}
