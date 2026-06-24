use crate::functions::deserialize_type_function_runtime_builder::deserialize_type_function_type_id_type_function_runtime_builder_state;
use crate::functions::get_type_function_runtime::get_type_function_runtime;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;
use luaur_vm::macros::lua_l_error::luaL_error;

pub unsafe fn is_subtype_of(l: *mut lua_State) -> i32 {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
        luaL_error!(
            vm_l,
            "type.issubtypeof: expected 2 arguments, but got {}",
            argument_count
        );
    }

    let self_ty = get_type_user_data(l, 1);
    let arg = get_type_user_data(l, 2);

    let runtime = get_type_function_runtime(l);
    let runtime_builder =
        unsafe { &mut *((*runtime).runtime_builder as *mut TypeFunctionRuntimeBuilderState) };
    let ctx = unsafe { &*runtime_builder.ctx };

    let sub_ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
        self_ty,
        runtime_builder as *mut TypeFunctionRuntimeBuilderState,
    );
    if !runtime_builder.errors.is_empty() || !runtime_builder.errors_deprecated.is_empty() {
        luaL_error!(vm_l, "failed to deserialize the self type");
    }

    let super_ty = deserialize_type_function_type_id_type_function_runtime_builder_state(
        arg,
        runtime_builder as *mut TypeFunctionRuntimeBuilderState,
    );
    if !runtime_builder.errors.is_empty() || !runtime_builder.errors_deprecated.is_empty() {
        luaL_error!(vm_l, "failed to deserialize the argument type");
    }

    let result = unsafe {
        (*ctx.subtyping.as_ptr()).is_subtype_type_id_type_id_not_null_scope(
            sub_ty,
            super_ty,
            ctx.scope.as_ptr(),
        )
    };
    lua_pushboolean(vm_l, result.is_subtype as core::ffi::c_int);
    1
}
