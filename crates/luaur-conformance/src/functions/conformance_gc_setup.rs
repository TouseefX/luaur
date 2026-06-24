use crate::functions::conformance_gc_set_block_allocations::conformance_gc_set_block_allocations;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_gc_setup(l: *mut lua_State) {
    lua_pushcclosurek(
        l,
        Some(conformance_gc_set_block_allocations),
        c"setblockallocations".as_ptr(),
        0,
        None,
    );
    lua_setglobal(l, c"setblockallocations".as_ptr());
}
