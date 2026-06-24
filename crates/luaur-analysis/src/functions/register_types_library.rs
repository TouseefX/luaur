//! Faithful port of `void registerTypesLibrary(lua_State* L)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1876-1914).
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_l_register::lua_l_register;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::records::lua_l_reg::LuaLReg;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

/// Generates a `lua_CFunction`-shaped thunk that forwards to the analysis-level
/// function declared over the `c_void` `lua_State` alias. `lua_CFunction` is
/// `unsafe fn(*mut vm::lua_State) -> c_int`, so each registered function needs a
/// thin bridging thunk.
macro_rules! type_lib_thunk {
    ($thunk:ident, $real:path) => {
        unsafe fn $thunk(l: *mut luaur_vm::records::lua_state::lua_State) -> core::ffi::c_int {
            $real(l as *mut lua_State)
        }
    };
}

type_lib_thunk!(
    create_unknown_thunk,
    crate::functions::create_unknown::create_unknown
);
type_lib_thunk!(
    create_never_thunk,
    crate::functions::create_never::create_never
);
type_lib_thunk!(create_any_thunk, crate::functions::create_any::create_any);
type_lib_thunk!(
    create_boolean_thunk,
    crate::functions::create_boolean::create_boolean
);
type_lib_thunk!(
    create_number_thunk,
    crate::functions::create_number::create_number
);
type_lib_thunk!(
    create_string_thunk,
    crate::functions::create_string::create_string
);
type_lib_thunk!(
    create_thread_thunk,
    crate::functions::create_thread::create_thread
);
type_lib_thunk!(
    create_buffer_thunk,
    crate::functions::create_buffer::create_buffer
);

type_lib_thunk!(
    create_singleton_thunk,
    crate::functions::create_singleton::create_singleton
);
type_lib_thunk!(
    create_negation_thunk,
    crate::functions::create_negation::create_negation
);
type_lib_thunk!(
    create_union_thunk,
    crate::functions::create_union::create_union
);
type_lib_thunk!(
    create_intersection_thunk,
    crate::functions::create_intersection::create_intersection
);
type_lib_thunk!(
    create_optional_thunk,
    crate::functions::create_optional::create_optional
);
type_lib_thunk!(
    create_table_thunk,
    crate::functions::create_table::create_table
);
type_lib_thunk!(
    create_function_thunk,
    crate::functions::create_function::create_function
);
type_lib_thunk!(deep_copy_thunk, crate::functions::deep_copy::deep_copy);
type_lib_thunk!(
    create_generic_thunk,
    crate::functions::create_generic::create_generic
);

pub unsafe fn register_types_library(l: *mut lua_State) {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;

    // luaL_Reg fields[] = { ... {nullptr, nullptr} };
    let fields: [LuaLReg; 9] = [
        LuaLReg {
            name: c"unknown".as_ptr(),
            func: Some(create_unknown_thunk),
        },
        LuaLReg {
            name: c"never".as_ptr(),
            func: Some(create_never_thunk),
        },
        LuaLReg {
            name: c"any".as_ptr(),
            func: Some(create_any_thunk),
        },
        LuaLReg {
            name: c"boolean".as_ptr(),
            func: Some(create_boolean_thunk),
        },
        LuaLReg {
            name: c"number".as_ptr(),
            func: Some(create_number_thunk),
        },
        LuaLReg {
            name: c"string".as_ptr(),
            func: Some(create_string_thunk),
        },
        LuaLReg {
            name: c"thread".as_ptr(),
            func: Some(create_thread_thunk),
        },
        LuaLReg {
            name: c"buffer".as_ptr(),
            func: Some(create_buffer_thunk),
        },
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
    ];

    // luaL_Reg methods[] = { ... {nullptr, nullptr} };
    let methods: [LuaLReg; 10] = [
        LuaLReg {
            name: c"singleton".as_ptr(),
            func: Some(create_singleton_thunk),
        },
        LuaLReg {
            name: c"negationof".as_ptr(),
            func: Some(create_negation_thunk),
        },
        LuaLReg {
            name: c"unionof".as_ptr(),
            func: Some(create_union_thunk),
        },
        LuaLReg {
            name: c"intersectionof".as_ptr(),
            func: Some(create_intersection_thunk),
        },
        LuaLReg {
            name: c"optional".as_ptr(),
            func: Some(create_optional_thunk),
        },
        LuaLReg {
            name: c"newtable".as_ptr(),
            func: Some(create_table_thunk),
        },
        LuaLReg {
            name: c"newfunction".as_ptr(),
            func: Some(create_function_thunk),
        },
        LuaLReg {
            name: c"copy".as_ptr(),
            func: Some(deep_copy_thunk),
        },
        LuaLReg {
            name: c"generic".as_ptr(),
            func: Some(create_generic_thunk),
        },
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
    ];

    // luaL_register(L, "types", methods);
    lua_l_register(vm_l, c"types".as_ptr(), methods.as_ptr());

    // Set fields for type userdata
    // for (luaL_Reg* l = fields; l->name; l++)
    let mut i = 0usize;
    while !fields[i].name.is_null() {
        // l->func(L);
        let func: lua_CFunction = fields[i].func;
        (func.unwrap())(vm_l);
        // lua_setfield(L, -2, l->name);
        lua_setfield(vm_l, -2, fields[i].name);
        i += 1;
    }

    // lua_pop(L, 1);
    lua_pop(vm_l, 1);
}
