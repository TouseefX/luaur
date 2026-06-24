use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::records::type_function_property::TypeFunctionProperty;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_string_singleton::TypeFunctionStringSingleton;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_singleton_variant::TypeFunctionSingletonVariant;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use alloc::collections::BTreeMap;
use alloc::string::String;
use luaur_vm::functions::lua_createtable::lua_createtable;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::functions::lua_settable::lua_settable;

pub unsafe fn get_props(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.properties: expected 1 arguments, but got {}",
                argument_count
            ),
        );
    }

    let self_ty = get_type_user_data(l, 1);

    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if !tftt.is_null() {
        push_props(l, vm_l, &(*tftt).props);
        return 1;
    }

    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if !tfct.is_null() {
        push_props(l, vm_l, &(*tfct).props);
        return 1;
    }

    lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
            "type.properties: expected self to be either a table or class, but got {} instead",
            get_tag(l, self_ty)
        ),
    );
    0
}

unsafe fn push_props(
    l: *mut lua_State,
    vm_l: *mut luaur_vm::records::lua_state::lua_State,
    props: &BTreeMap<String, TypeFunctionProperty>,
) {
    lua_createtable(vm_l, props.len() as core::ffi::c_int, 0);

    for (name, prop) in props {
        alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
                variant: TypeFunctionSingletonVariant::V1(TypeFunctionStringSingleton {
                    value: name.clone(),
                }),
            }),
            false,
        );

        let mut size: core::ffi::c_int = 0;
        if prop.read_ty.is_some() {
            size += 1;
        }
        if prop.write_ty.is_some() {
            size += 1;
        }

        lua_createtable(vm_l, 0, size);

        if let Some(read_ty) = prop.read_ty {
            alloc_type_user_data(l, (*read_ty).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"read".as_ptr());
        }

        if let Some(write_ty) = prop.write_ty {
            alloc_type_user_data(l, (*write_ty).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"write".as_ptr());
        }

        lua_settable(vm_l, -3);
    }
}
