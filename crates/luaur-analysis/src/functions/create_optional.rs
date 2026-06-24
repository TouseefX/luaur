use crate::enums::type_type_function_runtime::Type;
use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::allocate_type_function_type::allocate_type_function_type;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;
use crate::records::type_function_union_type::TypeFunctionUnionType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

pub unsafe fn create_optional(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "types.optional: expected 1 argument, but got {}",
                argument_count
            ),
        );
    }

    let argument: TypeFunctionTypeId = get_type_user_data(l, 1);

    let mut components: alloc::vec::Vec<TypeFunctionTypeId> = alloc::vec::Vec::new();

    let union_ty = get_type_function_type_id::<TypeFunctionUnionType>(argument);
    if !union_ty.is_null() {
        components.reserve((*union_ty).components.len() + 1);
        components.extend((*union_ty).components.iter().copied());
    } else {
        components.push(argument);
    }

    let nil_type = TypeFunctionPrimitiveType::new(Type::NilType);
    let nil_variant = TypeFunctionTypeVariant::Primitive(nil_type);
    let nil_id = allocate_type_function_type(l, nil_variant);
    components.push(nil_id);

    let union_type = TypeFunctionUnionType { components };
    let union_variant = TypeFunctionTypeVariant::Union(union_type);
    alloc_type_user_data(l, union_variant, false);

    1
}
