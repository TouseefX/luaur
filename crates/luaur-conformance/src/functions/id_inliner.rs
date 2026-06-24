use luaur_vm::records::closure::Closure;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::records::proto::Proto;

pub unsafe extern "C" fn id_inliner(
    _l: *mut lua_State,
    caller: *mut Closure,
    _target: *mut Closure,
    _pc: u32,
) -> *mut Proto {
    (&(*caller).inner.l).p
}
