use crate::functions::int_64_add::int_64_add;
use crate::functions::int_64_ctor::int_64_ctor;
use crate::functions::int_64_div::int_64_div;
use crate::functions::int_64_eq::int_64_eq;
use crate::functions::int_64_idiv::int_64_idiv;
use crate::functions::int_64_index::int_64_index;
use crate::functions::int_64_le::int_64_le;
use crate::functions::int_64_lt::int_64_lt;
use crate::functions::int_64_mod::int_64_mod;
use crate::functions::int_64_mul::int_64_mul;
use crate::functions::int_64_newindex::int_64_newindex;
use crate::functions::int_64_pow::int_64_pow;
use crate::functions::int_64_sub::int_64_sub;
use crate::functions::int_64_tostring::int_64_tostring;
use crate::functions::int_64_unm::int_64_unm;
use luaur_vm::functions::lua_l_newmetatable::lua_l_newmetatable;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

pub unsafe extern "C" fn conformance_userdata_setup(l: *mut lua_State) {
    lua_l_newmetatable(l, c"int64".as_ptr());

    let index: lua_CFunction = Some(int_64_index);
    lua_pushcclosurek(l, index, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__index".as_ptr());

    let newindex: lua_CFunction = Some(int_64_newindex);
    lua_pushcclosurek(l, newindex, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__newindex".as_ptr());

    let eq: lua_CFunction = Some(int_64_eq);
    lua_pushcclosurek(l, eq, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__eq".as_ptr());

    let lt: lua_CFunction = Some(int_64_lt);
    lua_pushcclosurek(l, lt, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__lt".as_ptr());

    let le: lua_CFunction = Some(int_64_le);
    lua_pushcclosurek(l, le, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__le".as_ptr());

    let add: lua_CFunction = Some(int_64_add);
    lua_pushcclosurek(l, add, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__add".as_ptr());

    let sub: lua_CFunction = Some(int_64_sub);
    lua_pushcclosurek(l, sub, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__sub".as_ptr());

    let mul: lua_CFunction = Some(int_64_mul);
    lua_pushcclosurek(l, mul, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__mul".as_ptr());

    let div: lua_CFunction = Some(int_64_div);
    lua_pushcclosurek(l, div, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__div".as_ptr());

    let idiv: lua_CFunction = Some(int_64_idiv);
    lua_pushcclosurek(l, idiv, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__idiv".as_ptr());

    let modulo: lua_CFunction = Some(int_64_mod);
    lua_pushcclosurek(l, modulo, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__mod".as_ptr());

    let pow: lua_CFunction = Some(int_64_pow);
    lua_pushcclosurek(l, pow, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__pow".as_ptr());

    let unm: lua_CFunction = Some(int_64_unm);
    lua_pushcclosurek(l, unm, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__unm".as_ptr());

    let tostring: lua_CFunction = Some(int_64_tostring);
    lua_pushcclosurek(l, tostring, core::ptr::null(), 0, None);
    lua_setfield(l, -2, c"__tostring".as_ptr());

    let ctor: lua_CFunction = Some(int_64_ctor);
    lua_pushcclosurek(l, ctor, c"int64".as_ptr(), 0, None);
    lua_setglobal(l, c"int64".as_ptr());
}
