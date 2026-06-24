use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_string::is_string;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::extern_type::ExternType;
use crate::records::metatable_type::MetatableType;
use crate::records::primitive_type::PrimitiveType;
use crate::type_aliases::type_id::TypeId;

pub fn get_metatable_type_id_not_null_builtin_types(
    type_: TypeId,
    builtin_types: &BuiltinTypes,
) -> Option<TypeId> {
    let type_ = unsafe { follow_type_id(type_) };

    let mt_type = unsafe { get_type_id::<MetatableType>(type_) };
    if !mt_type.is_null() {
        return Some(unsafe { (*mt_type).metatable });
    }

    let extern_type = unsafe { get_type_id::<ExternType>(type_) };
    if !extern_type.is_null() {
        return unsafe { (*extern_type).metatable };
    }

    if is_string(type_) {
        let ptv = unsafe { get_type_id::<PrimitiveType>(builtin_types.stringType) };
        unsafe {
            luaur_common::macros::luau_assert::LUAU_ASSERT!(!ptv.is_null());
            luaur_common::macros::luau_assert::LUAU_ASSERT!((*ptv).metatable.is_some());
            return (*ptv).metatable;
        }
    }

    None
}
