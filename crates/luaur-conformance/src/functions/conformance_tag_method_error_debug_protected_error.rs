use core::sync::atomic::Ordering;

use crate::functions::get_first_luau_frame_debug_info::get_first_luau_frame_debug_info;
use crate::records::conformance_tag_method_error_state::CONFORMANCE_TAG_METHOD_ERROR_STATE;
use luaur_vm::functions::lua_break::lua_break;
use luaur_vm::functions::lua_isyieldable::lua_isyieldable;
use luaur_vm::records::lua_state::lua_State;

const EXPECTED_HITS: [i32; 3] = [37, 54, 73];

pub unsafe extern "C-unwind" fn conformance_tag_method_error_debug_protected_error(
    l: *mut lua_State,
) {
    let ar = get_first_luau_frame_debug_info(l);

    assert_ne!(lua_isyieldable(l), 0);
    let ar = ar.expect("expected a Lua frame");

    let index = CONFORMANCE_TAG_METHOD_ERROR_STATE
        .index
        .fetch_add(1, Ordering::SeqCst);
    assert!((index as usize) < EXPECTED_HITS.len());
    assert_eq!(ar.currentline, EXPECTED_HITS[index as usize]);

    if CONFORMANCE_TAG_METHOD_ERROR_STATE
        .lua_break
        .load(Ordering::SeqCst)
    {
        lua_break(l);
    }
}
