use luaur_vm::macros::vm_protect_pc::VM_PROTECT_PC as VM_PROTECT_PC_VM;
use luaur_vm::type_aliases::lua_state::lua_State;

#[allow(non_snake_case)]
pub unsafe fn VM_PROTECT_PC(L: *mut lua_State, pc: *const u32) {
    VM_PROTECT_PC_VM(L, pc);
}
