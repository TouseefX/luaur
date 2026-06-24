//! Faithful port of `void registerTypeUserData(lua_State* L)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1932-2064).
//!
//! Creates and registers the `"type"` metatable for type userdata: installs
//! `__type`, `__metatable`, `__eq`, a method table (gated on
//! `LuauTypeFunctionRobustness`), an optional `issubtypeof` method (gated on
//! `LuauUdtfTypeIsSubtypeOf`), the dynamic `__index` closure, and the userdata
//! destructor.
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_l_newmetatable::lua_l_newmetatable;
use luaur_vm::functions::lua_l_register::lua_l_register;
use luaur_vm::functions::lua_pushstring::lua_pushstring;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::functions::lua_setreadonly::lua_setreadonly;
use luaur_vm::functions::lua_setuserdatadtor::lua_setuserdatadtor;
use luaur_vm::macros::lua_newtable::lua_newtable;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_pushcclosure::lua_pushcclosure;
use luaur_vm::macros::lua_pushcfunction::LUA_PUSHCFUNCTION;
use luaur_vm::records::lua_l_reg::LuaLReg;

// `kTypeUserdataTag` (Analysis/src/TypeFunctionRuntime.cpp:250).
const K_TYPE_USERDATA_TAG: i32 = 42;

/// Generates a `lua_CFunction`-shaped thunk forwarding to an analysis-level
/// function declared over the `c_void` `lua_State` alias.
macro_rules! tud_thunk {
    ($thunk:ident, $real:path) => {
        unsafe fn $thunk(l: *mut luaur_vm::records::lua_state::lua_State) -> core::ffi::c_int {
            $real(l as *mut lua_State)
        }
    };
}

tud_thunk!(check_tag_thunk, crate::functions::check_tag::check_tag);
tud_thunk!(
    get_negated_value_thunk,
    crate::functions::get_negated_value::get_negated_value
);
tud_thunk!(
    get_singleton_value_thunk,
    crate::functions::get_singleton_value::get_singleton_value
);
tud_thunk!(
    set_table_prop_thunk,
    crate::functions::set_table_prop::set_table_prop
);
tud_thunk!(
    set_read_table_prop_thunk,
    crate::functions::set_read_table_prop::set_read_table_prop
);
tud_thunk!(
    set_write_table_prop_thunk,
    crate::functions::set_write_table_prop::set_write_table_prop
);
tud_thunk!(
    read_table_prop_thunk,
    crate::functions::read_table_prop::read_table_prop
);
tud_thunk!(
    write_table_prop_thunk,
    crate::functions::write_table_prop::write_table_prop
);
tud_thunk!(get_props_thunk, crate::functions::get_props::get_props);
tud_thunk!(
    set_table_indexer_thunk,
    crate::functions::set_table_indexer::set_table_indexer
);
tud_thunk!(
    set_table_read_indexer_thunk,
    crate::functions::set_table_read_indexer::set_table_read_indexer
);
tud_thunk!(
    set_table_write_indexer_thunk,
    crate::functions::set_table_write_indexer::set_table_write_indexer
);
tud_thunk!(
    get_indexer_thunk,
    crate::functions::get_indexer::get_indexer
);
tud_thunk!(
    get_read_indexer_thunk,
    crate::functions::get_read_indexer::get_read_indexer
);
tud_thunk!(
    get_write_indexer_thunk,
    crate::functions::get_write_indexer::get_write_indexer
);
tud_thunk!(
    set_table_metatable_thunk,
    crate::functions::set_table_metatable::set_table_metatable
);
tud_thunk!(
    get_metatable_thunk,
    crate::functions::get_metatable_type_function_runtime::get_metatable
);
tud_thunk!(
    set_function_parameters_thunk,
    crate::functions::set_function_parameters::set_function_parameters
);
tud_thunk!(
    get_function_parameters_thunk,
    crate::functions::get_function_parameters::get_function_parameters
);
tud_thunk!(
    set_function_returns_thunk,
    crate::functions::set_function_returns::set_function_returns
);
tud_thunk!(
    get_function_returns_thunk,
    crate::functions::get_function_returns::get_function_returns
);
tud_thunk!(
    set_function_generics_thunk,
    crate::functions::set_function_generics::set_function_generics
);
tud_thunk!(
    get_function_generics_thunk,
    crate::functions::get_function_generics::get_function_generics
);
tud_thunk!(
    get_components_thunk,
    crate::functions::get_components::get_components
);
tud_thunk!(
    get_read_parent_thunk,
    crate::functions::get_read_parent::get_read_parent
);
tud_thunk!(
    get_write_parent_thunk,
    crate::functions::get_write_parent::get_write_parent
);
tud_thunk!(
    get_generic_name_thunk,
    crate::functions::get_generic_name::get_generic_name
);
tud_thunk!(
    get_generic_is_pack_thunk,
    crate::functions::get_generic_is_pack::get_generic_is_pack
);
tud_thunk!(
    is_equal_to_type_thunk,
    crate::functions::is_equal_to_type::is_equal_to_type
);
tud_thunk!(
    is_subtype_of_thunk,
    crate::functions::is_subtype_of::is_subtype_of
);
tud_thunk!(
    type_userdata_index_thunk,
    crate::functions::type_userdata_index::type_userdata_index
);

/// `extern "C"` destructor thunk for `deallocTypeUserData`. The VM's
/// `lua_Destructor` is `extern "C" fn(*mut vm::lua_State, *mut c_void)`.
unsafe extern "C" fn dealloc_type_user_data_thunk(
    l: *mut luaur_vm::records::lua_state::lua_State,
    data: *mut core::ffi::c_void,
) {
    crate::functions::dealloc_type_user_data::dealloc_type_user_data(l as *mut lua_State, data);
}

pub unsafe fn register_type_user_data(l: *mut lua_State) {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;

    // luaL_Reg typeUserdataMethods_DEPRECATED[] = { ... {nullptr, nullptr} };
    let type_userdata_methods_deprecated: [LuaLReg; 34] = [
        LuaLReg {
            name: c"is".as_ptr(),
            func: Some(check_tag_thunk),
        },
        // Negation type methods
        LuaLReg {
            name: c"inner".as_ptr(),
            func: Some(get_negated_value_thunk),
        },
        // Singleton type methods
        LuaLReg {
            name: c"value".as_ptr(),
            func: Some(get_singleton_value_thunk),
        },
        // Table type methods
        LuaLReg {
            name: c"setproperty".as_ptr(),
            func: Some(set_table_prop_thunk),
        },
        LuaLReg {
            name: c"setreadproperty".as_ptr(),
            func: Some(set_read_table_prop_thunk),
        },
        LuaLReg {
            name: c"setwriteproperty".as_ptr(),
            func: Some(set_write_table_prop_thunk),
        },
        LuaLReg {
            name: c"readproperty".as_ptr(),
            func: Some(read_table_prop_thunk),
        },
        LuaLReg {
            name: c"writeproperty".as_ptr(),
            func: Some(write_table_prop_thunk),
        },
        LuaLReg {
            name: c"properties".as_ptr(),
            func: Some(get_props_thunk),
        },
        LuaLReg {
            name: c"setindexer".as_ptr(),
            func: Some(set_table_indexer_thunk),
        },
        LuaLReg {
            name: c"setreadindexer".as_ptr(),
            func: Some(set_table_read_indexer_thunk),
        },
        LuaLReg {
            name: c"setwriteindexer".as_ptr(),
            func: Some(set_table_write_indexer_thunk),
        },
        LuaLReg {
            name: c"indexer".as_ptr(),
            func: Some(get_indexer_thunk),
        },
        LuaLReg {
            name: c"readindexer".as_ptr(),
            func: Some(get_read_indexer_thunk),
        },
        LuaLReg {
            name: c"writeindexer".as_ptr(),
            func: Some(get_write_indexer_thunk),
        },
        LuaLReg {
            name: c"setmetatable".as_ptr(),
            func: Some(set_table_metatable_thunk),
        },
        LuaLReg {
            name: c"metatable".as_ptr(),
            func: Some(get_metatable_thunk),
        },
        // Function type methods
        LuaLReg {
            name: c"setparameters".as_ptr(),
            func: Some(set_function_parameters_thunk),
        },
        LuaLReg {
            name: c"parameters".as_ptr(),
            func: Some(get_function_parameters_thunk),
        },
        LuaLReg {
            name: c"setreturns".as_ptr(),
            func: Some(set_function_returns_thunk),
        },
        LuaLReg {
            name: c"returns".as_ptr(),
            func: Some(get_function_returns_thunk),
        },
        LuaLReg {
            name: c"setgenerics".as_ptr(),
            func: Some(set_function_generics_thunk),
        },
        LuaLReg {
            name: c"generics".as_ptr(),
            func: Some(get_function_generics_thunk),
        },
        // Union and Intersection type methods
        LuaLReg {
            name: c"components".as_ptr(),
            func: Some(get_components_thunk),
        },
        // Extern type methods
        LuaLReg {
            name: c"readparent".as_ptr(),
            func: Some(get_read_parent_thunk),
        },
        LuaLReg {
            name: c"writeparent".as_ptr(),
            func: Some(get_write_parent_thunk),
        },
        // Function type methods (cont.)
        LuaLReg {
            name: c"setgenerics".as_ptr(),
            func: Some(set_function_generics_thunk),
        },
        LuaLReg {
            name: c"generics".as_ptr(),
            func: Some(get_function_generics_thunk),
        },
        // Generic type methods
        LuaLReg {
            name: c"name".as_ptr(),
            func: Some(get_generic_name_thunk),
        },
        LuaLReg {
            name: c"ispack".as_ptr(),
            func: Some(get_generic_is_pack_thunk),
        },
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
        // Padding to keep array length stable with the duplicate-entry layout
        // above (C++ relies on the {nullptr,nullptr} sentinel; trailing entries
        // past the sentinel are never read).
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
    ];

    // luaL_Reg typeUserdataMethods[] = { ... {nullptr, nullptr} };
    let type_userdata_methods: [LuaLReg; 31] = [
        LuaLReg {
            name: c"is".as_ptr(),
            func: Some(check_tag_thunk),
        },
        // Negation type methods
        LuaLReg {
            name: c"inner".as_ptr(),
            func: Some(get_negated_value_thunk),
        },
        // Singleton type methods
        LuaLReg {
            name: c"value".as_ptr(),
            func: Some(get_singleton_value_thunk),
        },
        // Table type methods
        LuaLReg {
            name: c"setproperty".as_ptr(),
            func: Some(set_table_prop_thunk),
        },
        LuaLReg {
            name: c"setreadproperty".as_ptr(),
            func: Some(set_read_table_prop_thunk),
        },
        LuaLReg {
            name: c"setwriteproperty".as_ptr(),
            func: Some(set_write_table_prop_thunk),
        },
        LuaLReg {
            name: c"readproperty".as_ptr(),
            func: Some(read_table_prop_thunk),
        },
        LuaLReg {
            name: c"writeproperty".as_ptr(),
            func: Some(write_table_prop_thunk),
        },
        LuaLReg {
            name: c"properties".as_ptr(),
            func: Some(get_props_thunk),
        },
        LuaLReg {
            name: c"setindexer".as_ptr(),
            func: Some(set_table_indexer_thunk),
        },
        LuaLReg {
            name: c"setreadindexer".as_ptr(),
            func: Some(set_table_read_indexer_thunk),
        },
        LuaLReg {
            name: c"setwriteindexer".as_ptr(),
            func: Some(set_table_write_indexer_thunk),
        },
        LuaLReg {
            name: c"indexer".as_ptr(),
            func: Some(get_indexer_thunk),
        },
        LuaLReg {
            name: c"readindexer".as_ptr(),
            func: Some(get_read_indexer_thunk),
        },
        LuaLReg {
            name: c"writeindexer".as_ptr(),
            func: Some(get_write_indexer_thunk),
        },
        LuaLReg {
            name: c"setmetatable".as_ptr(),
            func: Some(set_table_metatable_thunk),
        },
        LuaLReg {
            name: c"metatable".as_ptr(),
            func: Some(get_metatable_thunk),
        },
        // Function type methods
        LuaLReg {
            name: c"setparameters".as_ptr(),
            func: Some(set_function_parameters_thunk),
        },
        LuaLReg {
            name: c"parameters".as_ptr(),
            func: Some(get_function_parameters_thunk),
        },
        LuaLReg {
            name: c"setreturns".as_ptr(),
            func: Some(set_function_returns_thunk),
        },
        LuaLReg {
            name: c"returns".as_ptr(),
            func: Some(get_function_returns_thunk),
        },
        LuaLReg {
            name: c"setgenerics".as_ptr(),
            func: Some(set_function_generics_thunk),
        },
        LuaLReg {
            name: c"generics".as_ptr(),
            func: Some(get_function_generics_thunk),
        },
        // Union and Intersection type methods
        LuaLReg {
            name: c"components".as_ptr(),
            func: Some(get_components_thunk),
        },
        // Extern type methods
        LuaLReg {
            name: c"readparent".as_ptr(),
            func: Some(get_read_parent_thunk),
        },
        LuaLReg {
            name: c"writeparent".as_ptr(),
            func: Some(get_write_parent_thunk),
        },
        // Generic type methods
        LuaLReg {
            name: c"name".as_ptr(),
            func: Some(get_generic_name_thunk),
        },
        LuaLReg {
            name: c"ispack".as_ptr(),
            func: Some(get_generic_is_pack_thunk),
        },
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
        // Padding (see note above).
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
    ];

    // Create and register metatable for type userdata
    // luaL_newmetatable(L, "type");
    lua_l_newmetatable(vm_l, c"type".as_ptr());

    // lua_pushstring(L, "type"); lua_setfield(L, -2, "__type");
    lua_pushstring(vm_l, c"type".as_ptr());
    lua_setfield(vm_l, -2, c"__type".as_ptr());

    // Protect metatable from being changed
    // lua_pushstring(L, "The metatable is locked"); lua_setfield(L, -2, "__metatable");
    lua_pushstring(vm_l, c"The metatable is locked".as_ptr());
    lua_setfield(vm_l, -2, c"__metatable".as_ptr());

    // lua_pushcfunction(L, isEqualToType, "__eq"); lua_setfield(L, -2, "__eq");
    LUA_PUSHCFUNCTION(vm_l, Some(is_equal_to_type_thunk), c"__eq".as_ptr());
    lua_setfield(vm_l, -2, c"__eq".as_ptr());

    // Indexing will be a dynamic function because some type fields are dynamic
    // lua_newtable(L);
    lua_newtable(vm_l);
    // luaL_register(L, nullptr, FFlag::LuauTypeFunctionRobustness ? typeUserdataMethods : typeUserdataMethods_DEPRECATED);
    if luaur_common::FFlag::LuauTypeFunctionRobustness.get() {
        lua_l_register(vm_l, core::ptr::null(), type_userdata_methods.as_ptr());
    } else {
        lua_l_register(
            vm_l,
            core::ptr::null(),
            type_userdata_methods_deprecated.as_ptr(),
        );
    }

    // if (FFlag::LuauUdtfTypeIsSubtypeOf)
    if luaur_common::FFlag::LuauUdtfTypeIsSubtypeOf.get() {
        // lua_pushcfunction(L, isSubtypeOf, "issubtypeof"); lua_setfield(L, -2, "issubtypeof");
        LUA_PUSHCFUNCTION(vm_l, Some(is_subtype_of_thunk), c"issubtypeof".as_ptr());
        lua_setfield(vm_l, -2, c"issubtypeof".as_ptr());
    }

    // lua_setreadonly(L, -1, true);
    lua_setreadonly(vm_l, -1, 1);
    // lua_pushcclosure(L, typeUserdataIndex, "__index", 1);
    lua_pushcclosure(
        vm_l,
        Some(type_userdata_index_thunk),
        c"__index".as_ptr(),
        1,
    );
    // lua_setfield(L, -2, "__index");
    lua_setfield(vm_l, -2, c"__index".as_ptr());

    // lua_setreadonly(L, -1, true);
    lua_setreadonly(vm_l, -1, 1);
    // lua_pop(L, 1);
    lua_pop(vm_l, 1);

    // Sets up a destructor for the type userdata.
    // lua_setuserdatadtor(L, kTypeUserdataTag, deallocTypeUserData);
    lua_setuserdatadtor(
        vm_l,
        K_TYPE_USERDATA_TAG,
        Some(dealloc_type_user_data_thunk),
    );
}
