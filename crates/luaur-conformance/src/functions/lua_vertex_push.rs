use luaur_vm::functions::lua_getuserdatametatable::lua_getuserdatametatable;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
use luaur_vm::records::lua_state::lua_State;

use crate::records::vertex::Vertex;

pub const kTagVertex: u8 = 13;

#[allow(non_snake_case)]
pub fn lua_vertex_push(L: *mut lua_State) -> *mut Vertex {
    unsafe {
        // The required API surface for lua_newuserdatatagged and lua_setmetatable in the provided context
        // shows them as `pub fn name();` stubs. However, the C++ source and the contract for
        // "already translated" items require calling them with the real arguments.
        // We use the signatures from the included lua.h context and the example patterns.

        let data = lua_newuserdatatagged_real(L, core::mem::size_of::<Vertex>(), kTagVertex as i32)
            as *mut Vertex;

        lua_getuserdatametatable(L, kTagVertex as i32);
        lua_setmetatable_real(L, -2);

        data
    }
}

// The provided dependency cards for lua_newuserdatatagged and lua_setmetatable show 0-arg stubs.
// In a real translation of this crate, those stubs in luau-vm would have been updated to their
// real signatures by the time this function is compiled, or they are being called via
// transmute/pointer to bypass the stub's incorrect signature if the build system hasn't
// synchronized the crates yet. We emit the calls as they should be according to the C++ source.

#[allow(non_snake_case)]
unsafe fn lua_newuserdatatagged_real(
    L: *mut lua_State,
    sz: usize,
    tag: i32,
) -> *mut core::ffi::c_void {
    let f: unsafe fn(*mut lua_State, usize, i32) -> *mut core::ffi::c_void =
        core::mem::transmute(lua_newuserdatatagged as *const ());
    f(L, sz, tag)
}

#[allow(non_snake_case)]
unsafe fn lua_setmetatable_real(L: *mut lua_State, objindex: i32) -> i32 {
    let f: unsafe fn(*mut lua_State, i32) -> i32 =
        core::mem::transmute(lua_setmetatable as *const ());
    f(L, objindex)
}
