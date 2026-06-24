use core::ffi::c_int;

use crate::functions::multiple_yields::multipleYields;
use crate::functions::multiple_yields_continuation::multipleYieldsContinuation;
use crate::functions::multiple_yields_with_nested_call::multipleYieldsWithNestedCall;
use crate::functions::multiple_yields_with_nested_call_continuation::multiple_yields_with_nested_call_continuation;
use crate::functions::passthrough_call::passthrough_call;
use crate::functions::passthrough_call_arg_reuse::passthrough_call_arg_reuse;
use crate::functions::passthrough_call_arg_reuse_continuation::passthrough_call_arg_reuse_continuation;
use crate::functions::passthrough_call_continuation::passthrough_call_continuation;
use crate::functions::passthrough_call_more_results::passthrough_call_more_results;
use crate::functions::passthrough_call_more_results_continuation::passthrough_call_more_results_continuation;
use crate::functions::passthrough_call_varadic::passthrough_call_varadic;
use crate::functions::passthrough_call_varadic_continuation::passthrough_call_varadic_continuation;
use crate::functions::passthrough_call_with_state::passthrough_call_with_state;
use crate::functions::passthrough_call_with_state_continuation::passthrough_call_with_state_continuation;
use crate::functions::single_yield::single_yield;
use crate::functions::single_yield_continuation::single_yield_continuation;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;
use luaur_vm::type_aliases::lua_continuation::lua_Continuation;

pub unsafe extern "C" fn conformance_c_yield_setup(l: *mut lua_State) {
    let single_yield_fn: lua_CFunction = Some(core::mem::transmute(
        single_yield as fn(*mut lua_State) -> c_int,
    ));
    let single_yield_cont: lua_Continuation = Some(core::mem::transmute(
        single_yield_continuation as fn(*mut lua_State, c_int) -> c_int,
    ));
    lua_pushcclosurek(
        l,
        single_yield_fn,
        c"singleYield".as_ptr(),
        0,
        single_yield_cont,
    );
    lua_setglobal(l, c"singleYield".as_ptr());

    lua_pushcclosurek(
        l,
        Some(multipleYields),
        c"multipleYields".as_ptr(),
        0,
        Some(multipleYieldsContinuation),
    );
    lua_setglobal(l, c"multipleYields".as_ptr());

    lua_pushcclosurek(
        l,
        Some(multipleYieldsWithNestedCall),
        c"multipleYieldsWithNestedCall".as_ptr(),
        0,
        Some(multiple_yields_with_nested_call_continuation),
    );
    lua_setglobal(l, c"multipleYieldsWithNestedCall".as_ptr());

    lua_pushcclosurek(
        l,
        Some(core::mem::transmute(
            passthrough_call as unsafe fn(*mut lua_State) -> c_int,
        )),
        c"passthroughCall".as_ptr(),
        0,
        Some(passthrough_call_continuation),
    );
    lua_setglobal(l, c"passthroughCall".as_ptr());

    lua_pushcclosurek(
        l,
        Some(passthrough_call_more_results),
        c"passthroughCallMoreResults".as_ptr(),
        0,
        Some(passthrough_call_more_results_continuation),
    );
    lua_setglobal(l, c"passthroughCallMoreResults".as_ptr());

    lua_pushcclosurek(
        l,
        Some(passthrough_call_arg_reuse),
        c"passthroughCallArgReuse".as_ptr(),
        0,
        Some(passthrough_call_arg_reuse_continuation),
    );
    lua_setglobal(l, c"passthroughCallArgReuse".as_ptr());

    lua_pushcclosurek(
        l,
        Some(passthrough_call_varadic),
        c"passthroughCallVaradic".as_ptr(),
        0,
        Some(passthrough_call_varadic_continuation),
    );
    lua_setglobal(l, c"passthroughCallVaradic".as_ptr());

    lua_pushcclosurek(
        l,
        Some(passthrough_call_with_state),
        c"passthroughCallWithState".as_ptr(),
        0,
        Some(passthrough_call_with_state_continuation),
    );
    lua_setglobal(l, c"passthroughCallWithState".as_ptr());
}
