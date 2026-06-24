use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::functions::push_type::push_type;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;
use crate::records::type_function_unknown_type::TypeFunctionUnknownType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use alloc::vec::Vec;
use luaur_vm::functions::lua_gettop::lua_gettop;

#[allow(unused_variables)]
pub unsafe fn create_intersection(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let arg_size = lua_gettop(vm_l);

    let mut components: Vec<TypeFunctionTypeId> = Vec::with_capacity(arg_size as usize);

    for i in 1..=arg_size {
        let component = get_type_user_data(l, i);

        if let Some(intersection_component) =
            get_type_function_type_id::<TypeFunctionIntersectionType>(component).as_ref()
        {
            components.extend(intersection_component.components.iter().copied());
        } else if !get_type_function_type_id::<TypeFunctionUnknownType>(component).is_null() {
            continue;
        } else {
            components.push(component);
        }
    }

    if components.is_empty() {
        alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Unknown(TypeFunctionUnknownType::default()),
            false,
        );
    } else if components.len() == 1 {
        push_type(l, components[0]);
    } else {
        alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Intersection(TypeFunctionIntersectionType { components }),
            false,
        );
    }

    1
}
