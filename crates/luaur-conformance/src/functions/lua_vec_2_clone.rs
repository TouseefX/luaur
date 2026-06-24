use crate::functions::lua_vec_2_push::lua_vec_2_push;
use crate::records::vec_2_conformance_ir_hooks::Vec2;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn lua_vec_2_clone(L: *mut lua_State, self_ptr: *mut Vec2) -> i32 {
    unsafe {
        let r_ptr = lua_vec_2_push(L);

        let self_val = &*self_ptr;
        let r_val = &mut *r_ptr;

        r_val.x = self_val.x;
        r_val.y = self_val.y;
    }
    1
}
