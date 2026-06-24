use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_extern_type::TypeFunctionExternType;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_createtable::lua_createtable;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushnil::lua_pushnil;
use luaur_vm::functions::lua_setfield::lua_setfield;

pub unsafe fn get_indexer(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.indexer: expected 1 arguments, but got {}",
                argument_count
            ),
        );
    }

    let self_ty = get_type_user_data(l, 1);

    let tftt = get_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if !tftt.is_null() {
        if (*tftt).indexer.is_none() {
            lua_pushnil(vm_l);
        } else {
            lua_createtable(vm_l, 0, 3);

            let indexer = (*tftt).indexer.as_ref().unwrap();
            alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"index".as_ptr());
            alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"readresult".as_ptr());
            alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"writeresult".as_ptr());
        }

        return 1;
    }

    let tfct = get_type_function_type_id::<TypeFunctionExternType>(self_ty);
    if !tfct.is_null() {
        if (*tfct).indexer.is_none() {
            lua_pushnil(vm_l);
        } else {
            lua_createtable(vm_l, 0, 3);

            let indexer = (*tfct).indexer.as_ref().unwrap();
            alloc_type_user_data(l, (*indexer.key_type).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"index".as_ptr());
            alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"readresult".as_ptr());
            alloc_type_user_data(l, (*indexer.value_type).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"writeresult".as_ptr());
        }

        return 1;
    }

    lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
            "type.indexer: self to be either a table or class, but got {} instead",
            get_tag(l, self_ty)
        ),
    );
    0
}
