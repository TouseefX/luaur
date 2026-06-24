use crate::macros::vm_reg::VM_REG;
use luaur_vm::functions::lua_d_performcally::lua_d_performcally;
use luaur_vm::macros::setobj_2_s::setobj_2_s as setobj2s;
use luaur_vm::macros::ttisnil::ttisnil;

use crate::type_aliases::lua_state::LuaState;
use luaur_vm::type_aliases::t_value::TValue;

pub unsafe fn forg_loop_non_table_fallback(
    L: *mut luaur_vm::records::lua_state::lua_State,
    insn_a: i32,
    aux: i32,
) -> i32 {
    let l_ptr = L;
    let base: *mut TValue = (*l_ptr).base;
    let mut ra: *mut TValue = VM_REG!(insn_a, l_ptr, base);

    // note: it's safe to push arguments past top for complicated reasons (see lvmexecute.cpp)
    setobj2s!(l_ptr, ra.add(3 + 2), ra.add(2));
    setobj2s!(l_ptr, ra.add(3 + 1), ra.add(1));
    setobj2s!(l_ptr, ra.add(3), ra);

    (*l_ptr).top = ra.add(3 + 3); // func + 2 args (state and index)
    luaur_common::LUAU_ASSERT!((*l_ptr).top <= (*l_ptr).stack_last);

    // The provided signature for lua_d_performcally in the dependency card was a stub: pub fn lua_d_performcally();
    // However, the C++ source and the error message indicate it must take (L, func, nresults) and return bool.
    // We use the signature required by the call site.
    let perform_call: unsafe extern "C" fn(
        *mut luaur_vm::records::lua_state::lua_State,
        *mut TValue,
        i32,
    ) -> bool = core::mem::transmute(lua_d_performcally as *const ());

    if perform_call(l_ptr, ra.add(3), aux as u8 as i32) {
        return -1; // yield/break, caller must exit native execution
    }

    (*l_ptr).top = (*(*l_ptr).ci).top;

    // recompute ra since stack might have been reallocated
    let base = (*l_ptr).base;
    ra = VM_REG!(insn_a, l_ptr, base);

    // copy first variable back into the iteration index
    setobj2s!(l_ptr, ra.add(2), ra.add(3));

    if ttisnil!(ra.add(3)) {
        0
    } else {
        1
    }
}

#[no_mangle]
pub unsafe extern "C" fn forgLoopNonTableFallback(
    L: *mut luaur_vm::records::lua_state::LuaState,
    insn_a: i32,
    aux: i32,
) -> i32 {
    forg_loop_non_table_fallback(L, insn_a, aux)
}
