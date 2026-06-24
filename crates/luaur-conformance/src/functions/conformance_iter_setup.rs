use crate::functions::c_yielding_iterator::c_yielding_iterator;
use crate::functions::c_yielding_iterator_continuation::c_yielding_iterator_continuation;
use crate::functions::setup_native_helpers::setup_native_helpers;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;
use luaur_vm::type_aliases::lua_continuation::lua_Continuation;

pub unsafe extern "C" fn conformance_iter_setup(l: *mut lua_State) {
    setup_native_helpers(l);

    let iterator: lua_CFunction = Some(c_yielding_iterator);
    let continuation: lua_Continuation = Some(c_yielding_iterator_continuation);

    lua_pushcclosurek(l, iterator, c"cYieldingIterator".as_ptr(), 0, continuation);
    lua_setglobal(l, c"cYieldingIterator".as_ptr());
}
