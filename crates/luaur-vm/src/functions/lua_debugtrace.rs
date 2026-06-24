//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:630:lua_debugtrace`
//! Source: `VM/src/ldebug.cpp:630-677` (hand-ported)

use crate::functions::append::append;
use crate::functions::lua_getinfo::lua_getinfo;
use crate::records::lua_debug::LuaDebug;
use crate::type_aliases::lua_state::lua_State;

static mut BUF: [core::ffi::c_char; 4096] = [0; 4096];

#[allow(non_snake_case)]
pub unsafe fn lua_debugtrace(L: *mut lua_State) -> *const core::ffi::c_char {
    const LIMIT1: core::ffi::c_int = 10;
    const LIMIT2: core::ffi::c_int = 10;

    let depth: core::ffi::c_int = (*L).ci.offset_from((*L).base_ci) as core::ffi::c_int;
    let mut offset: usize = 0;

    let mut ar: LuaDebug = core::mem::zeroed();

    extern "C" {
        fn snprintf(
            s: *mut core::ffi::c_char,
            n: usize,
            format: *const core::ffi::c_char,
            ...
        ) -> core::ffi::c_int;
    }

    let mut level: core::ffi::c_int = 0;
    while lua_getinfo(L, level, c"sln".as_ptr(), &mut ar as *mut LuaDebug) != 0 {
        if !ar.short_src.is_null() {
            offset = append(BUF.as_mut_ptr(), BUF.len(), offset, ar.short_src);
        }

        if ar.currentline > 0 {
            let mut line: [core::ffi::c_char; 32] = [0; 32];
            snprintf(
                line.as_mut_ptr(),
                line.len(),
                c":%d".as_ptr(),
                ar.currentline,
            );

            offset = append(BUF.as_mut_ptr(), BUF.len(), offset, line.as_ptr());
        }

        if !ar.name.is_null() {
            offset = append(BUF.as_mut_ptr(), BUF.len(), offset, c" function ".as_ptr());
            offset = append(BUF.as_mut_ptr(), BUF.len(), offset, ar.name);
        }

        offset = append(BUF.as_mut_ptr(), BUF.len(), offset, c"\n".as_ptr());

        if depth > LIMIT1 + LIMIT2 && level == LIMIT1 - 1 {
            let mut skip: [core::ffi::c_char; 32] = [0; 32];
            snprintf(
                skip.as_mut_ptr(),
                skip.len(),
                c"... (+%d frames)\n".as_ptr(),
                depth - LIMIT1 - LIMIT2,
            );

            offset = append(BUF.as_mut_ptr(), BUF.len(), offset, skip.as_ptr());

            level = depth - LIMIT2 - 1;
        }

        level += 1;
    }

    luaur_common::macros::luau_assert::LUAU_ASSERT!(offset < BUF.len());
    BUF[offset] = 0;

    BUF.as_ptr()
}
