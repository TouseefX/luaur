use luaur_vm::functions::lua_getuserdatametatable::lua_getuserdatametatable;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
use luaur_vm::records::lua_state::lua_State;

use crate::records::vec_2_conformance_ir_hooks::Vec2;

pub const kTagVec2: i32 = 12;

#[allow(non_snake_case)]
pub fn lua_vec_2_push(L: *mut lua_State) -> *mut Vec2 {
    unsafe {
        // The required context for lua_newuserdatatagged and lua_setmetatable shows empty signatures in the stub,
        // but the C++ source and the lua.h fragments confirm they take arguments.
        // We must cast the function pointers to the correct variadic-like signature to call them with arguments
        // as the Rust stubs for these VM functions are currently incomplete/skeleton stubs.

        type NewUserdataTaggedFn =
            unsafe extern "C" fn(*mut lua_State, usize, core::ffi::c_int) -> *mut core::ffi::c_void;
        let lua_newuserdatatagged_ptr: NewUserdataTaggedFn =
            core::mem::transmute(lua_newuserdatatagged as *const ());

        let data = lua_newuserdatatagged_ptr(
            L,
            core::mem::size_of::<Vec2>(),
            kTagVec2 as core::ffi::c_int,
        ) as *mut Vec2;

        lua_getuserdatametatable(L, kTagVec2 as core::ffi::c_int);

        type SetMetatableFn =
            unsafe extern "C" fn(*mut lua_State, core::ffi::c_int) -> core::ffi::c_int;
        let lua_setmetatable_ptr: SetMetatableFn =
            core::mem::transmute(lua_setmetatable as *const ());

        lua_setmetatable_ptr(L, -2);

        data
    }
}
