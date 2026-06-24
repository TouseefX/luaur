use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::records::type_function_type_pack::TypeFunctionTypePack;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use luaur_vm::functions::lua_createtable::lua_createtable;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_rawseti::lua_rawseti;
use luaur_vm::functions::lua_setfield::lua_setfield;

pub unsafe fn push_type_pack(l: *mut lua_State, tp: TypeFunctionTypePackId) {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;

    let tftp = get_type_function_type_pack_id::<TypeFunctionTypePack>(tp);
    if !tftp.is_null() {
        lua_createtable(vm_l, 0, 2);

        if !(*tftp).head.is_empty() {
            lua_createtable(vm_l, (*tftp).head.len() as i32, 0);
            let mut pos: i32 = 1;

            for el in (*tftp).head.iter() {
                alloc_type_user_data(l, (**el).type_variant.clone(), false);
                lua_rawseti(vm_l, -2, pos);
                pos += 1;
            }

            lua_setfield(vm_l, -2, c"head".as_ptr());
        }

        if let Some(tail) = (*tftp).tail {
            push_type_pack_tail(l, vm_l, tail);
            lua_setfield(vm_l, -2, c"tail".as_ptr());
        }
    } else {
        let tfvp = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tp);
        if !tfvp.is_null() {
            lua_createtable(vm_l, 0, 1);

            alloc_type_user_data(l, (*(*tfvp).type_id).type_variant.clone(), false);
            lua_setfield(vm_l, -2, c"tail".as_ptr());
        } else {
            let tfgp = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tp);
            if !tfgp.is_null() {
                lua_createtable(vm_l, 0, 1);

                alloc_type_user_data(
                    l,
                    TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
                        is_named: (*tfgp).is_named,
                        is_pack: true,
                        name: (*tfgp).name.clone(),
                    }),
                    false,
                );
                lua_setfield(vm_l, -2, c"tail".as_ptr());
            } else {
                lua_l_error_l(
                    vm_l,
                    c"%s".as_ptr(),
                    core::format_args!("unsupported type pack type"),
                );
            }
        }
    }
}

unsafe fn push_type_pack_tail(
    l: *mut lua_State,
    vm_l: *mut luaur_vm::records::lua_state::lua_State,
    tail: TypeFunctionTypePackId,
) {
    let tfvp = get_type_function_type_pack_id::<TypeFunctionVariadicTypePack>(tail);
    if !tfvp.is_null() {
        alloc_type_user_data(l, (*(*tfvp).type_id).type_variant.clone(), false);
        return;
    }

    let tfgp = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(tail);
    if !tfgp.is_null() {
        alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
                is_named: (*tfgp).is_named,
                is_pack: true,
                name: (*tfgp).name.clone(),
            }),
            false,
        );
        return;
    }

    lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!("unsupported type pack type"),
    );
}
