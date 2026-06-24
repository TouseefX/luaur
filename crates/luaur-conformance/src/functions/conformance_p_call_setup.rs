use core::ffi::c_int;

use crate::functions::conformance_p_call_resume_error::conformance_p_call_resume_error;
use crate::functions::cxxthrow::cxxthrow;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

pub unsafe extern "C" fn conformance_p_call_setup(l: *mut lua_State) {
    let cxxthrow_fn: lua_CFunction = Some(core::mem::transmute(
        cxxthrow as fn(*mut lua_State) -> c_int,
    ));
    lua_pushcclosurek(l, cxxthrow_fn, c"cxxthrow".as_ptr(), 0, None);
    lua_setglobal(l, c"cxxthrow".as_ptr());

    lua_pushcclosurek(
        l,
        Some(conformance_p_call_resume_error),
        c"resumeerror".as_ptr(),
        0,
        None,
    );
    lua_setglobal(l, c"resumeerror".as_ptr());
}
