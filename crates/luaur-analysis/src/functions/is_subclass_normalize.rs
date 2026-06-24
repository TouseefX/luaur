use crate::functions::get_type_alt_j::get_type_id;
use crate::records::extern_type::ExternType;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn is_subclass_type_id_type_id(
    test: crate::type_aliases::type_id::TypeId,
    parent: crate::type_aliases::type_id::TypeId,
) -> bool {
    let test_ctv = unsafe { get_type_id::<ExternType>(test) };
    let parent_ctv = unsafe { get_type_id::<ExternType>(parent) };

    LUAU_ASSERT!(!test_ctv.is_null());
    LUAU_ASSERT!(!parent_ctv.is_null());

    unsafe {
        crate::functions::is_subclass_type::is_subclass_extern_type_extern_type(
            &*test_ctv,
            &*parent_ctv,
        )
    }
}
