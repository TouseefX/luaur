use crate::functions::get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_never_type::TypeFunctionNeverType;
use crate::records::type_function_table_indexer::TypeFunctionTableIndexer;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

pub unsafe fn set_table_indexer(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 3 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setindexer: expected 3 arguments, but got {}",
                argument_count
            ),
        );
    }

    let self_ty = get_type_user_data(l, 1);
    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);

    if tftt.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setindexer: expected self to be either a table, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    if luaur_common::FFlag::LuauTypeFunctionSupportsFrozen.get() && (*self_ty).frozen {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setindexer: cannot be called to mutate a frozen type, use `types.copy` to make a copy"
            ),
        );
    }

    let key = get_type_user_data(l, 2);
    let value = get_type_user_data(l, 3);

    if !get_type_function_type_id::<TypeFunctionNeverType>(key).is_null() {
        (*tftt).indexer = None;
        return 0;
    }

    (*tftt).indexer = Some(TypeFunctionTableIndexer::new(key, value));
    0
}
