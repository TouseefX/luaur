use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_n::get_type_function_type_pack_id;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_vm::functions::lua_createtable::lua_createtable;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_rawseti::lua_rawseti;

pub unsafe fn get_function_generics(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let self_ty = get_type_user_data(l, 1);

    let tfft = get_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
    if tfft.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.generics: expected self to be a function, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    lua_createtable(
        vm_l,
        ((*tfft).generics.len() + (*tfft).generic_packs.len()) as i32,
        0,
    );

    let mut pos: i32 = 1;

    for el in &(*tfft).generics {
        alloc_type_user_data(l, (*(*el)).type_variant.clone(), false);
        lua_rawseti(vm_l, -2, pos);
        pos += 1;
    }

    for el in &(*tfft).generic_packs {
        let gty = get_type_function_type_pack_id::<TypeFunctionGenericTypePack>(*el);
        LUAU_ASSERT!(!gty.is_null());
        alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Generic(TypeFunctionGenericType {
                is_named: (*gty).is_named,
                is_pack: true,
                name: (*gty).name.clone(),
            }),
            false,
        );
        lua_rawseti(vm_l, -2, pos);
        pos += 1;
    }

    1
}
