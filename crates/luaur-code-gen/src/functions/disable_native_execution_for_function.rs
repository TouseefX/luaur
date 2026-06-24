use crate::functions::on_destroy_function::on_destroy_function;
use crate::type_aliases::lua_state::lua_State;

use luaur_vm::macros::clvalue::clvalue;
use luaur_vm::macros::ttisfunction::ttisfunction;
use luaur_vm::type_aliases::t_value::TValue;

#[allow(non_snake_case)]
pub fn disable_native_execution_for_function(L: *mut lua_State, level: i32) {
    unsafe {
        if L.is_null() {
            return;
        }

        // CODEGEN_ASSERT(unsigned(level) < unsigned(L->ci - L->base_ci));
        let ci = (*L).ci;
        let base_ci = (*L).base_ci;

        let diff = ci.offset_from(base_ci);
        if !(level as u32) < diff as u32 {
            // CODEGEN_ASSERT should abort via handler; keep behavior conservative if it doesn't.
            return;
        }

        // const CallInfo* ci = L->ci - level;
        let ci_ptr = ci.offset(-(level as isize));

        // const TValue* o = ci->func;
        let o = (*ci_ptr).func as *const TValue;

        // CODEGEN_ASSERT(ttisfunction(o));
        if !ttisfunction!(o) {
            return;
        }

        // Proto* proto = clvalue(o)->l.p;
        let cl = clvalue!(o);
        let proto = (*(*cl).inner.l).p;

        if proto.is_null() {
            return;
        }

        // CODEGEN_ASSERT(proto->codeentry != proto->code);
        if (*proto).codeentry == (*proto).code {
            return;
        }

        on_destroy_function(L, proto);
    }
}
