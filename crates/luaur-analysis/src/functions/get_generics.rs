//! Faithful port of
//! `static std::tuple<std::vector<TypeFunctionTypeId>, std::vector<TypeFunctionTypePackId>>
//!  getGenerics(lua_State* L, int idx, const char* fname)`
//! (Analysis/src/TypeFunctionRuntime.cpp:1125-1177).
use crate::functions::allocate_type_function_type_pack::allocate_type_function_type_pack;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_function_type_pack_variant::TypeFunctionTypePackVariant;
use alloc::vec::Vec;
use luaur_vm::functions::lua_gettable::lua_gettable;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_l_typeerror_l::lua_l_typeerror_l;
use luaur_vm::functions::lua_objlen::lua_objlen;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::macros::lua_isnil::lua_isnil;
use luaur_vm::macros::lua_isnoneornil::lua_isnoneornil;
use luaur_vm::macros::lua_istable::lua_istable;
use luaur_vm::macros::lua_pop::lua_pop;

pub unsafe fn get_generics(
    l: *mut lua_State,
    idx: i32,
    fname: &str,
) -> (Vec<TypeFunctionTypeId>, Vec<TypeFunctionTypePackId>) {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;

    let mut types: Vec<TypeFunctionTypeId> = Vec::new();
    let mut packs: Vec<TypeFunctionTypePackId> = Vec::new();

    if lua_istable!(vm_l, idx) {
        lua_pushvalue(vm_l, idx);

        let mut i: core::ffi::c_int = 1;
        while i <= lua_objlen(vm_l, -1) {
            lua_pushinteger(vm_l, i);
            lua_gettable(vm_l, -2);

            if lua_isnil!(vm_l, -1) {
                lua_pop(vm_l, 1);
                break;
            }

            // TypeFunctionTypeId ty = getTypeUserData(L, -1);
            let ty = get_type_user_data(l, -1);

            // if (auto gty = get<TypeFunctionGenericType>(ty))
            let gty = get_type_function_type_id::<TypeFunctionGenericType>(ty);
            if !gty.is_null() {
                if (*gty).is_pack {
                    packs.push(allocate_type_function_type_pack(
                        l,
                        TypeFunctionTypePackVariant::V2(TypeFunctionGenericTypePack {
                            is_named: (*gty).is_named,
                            name: (*gty).name.clone(),
                        }),
                    ));
                } else {
                    if !packs.is_empty() {
                        lua_l_error_l(
                            vm_l,
                            c"%s".as_ptr(),
                            core::format_args!(
                                "{}: generic type cannot follow a generic pack",
                                fname
                            ),
                        );
                    }

                    types.push(ty);
                }
            } else {
                lua_l_error_l(
                    vm_l,
                    c"%s".as_ptr(),
                    core::format_args!("{}: table member was not a generic type", fname),
                );
            }

            lua_pop(vm_l, 1);
            i += 1;
        }

        lua_pop(vm_l, 1);
    } else if !lua_isnoneornil!(vm_l, idx) {
        lua_l_typeerror_l(vm_l, idx, "table");
    }

    (types, packs)
}
