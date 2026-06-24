use crate::enums::type_type_function_runtime::Type;
use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;

#[allow(non_snake_case)]
pub unsafe fn create_thread(l: *mut lua_State) -> core::ffi::c_int {
    let primitive = TypeFunctionPrimitiveType::new(Type::Thread);
    alloc_type_user_data(l, TypeFunctionTypeVariant::Primitive(primitive), false);

    1
}
