use crate::records::extern_type::ExternType;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn is_subclass_extern_type_extern_type(cls: &ExternType, parent: &ExternType) -> bool {
    let mut current = cls;

    loop {
        if current as *const ExternType == parent as *const ExternType {
            return true;
        }

        let parent_id = match current.parent {
            Some(id) => id,
            None => return false,
        };

        let next =
            unsafe { crate::functions::get_type_alt_j::get_type_id::<ExternType>(parent_id) };
        LUAU_ASSERT!(!next.is_null());
        let next = unsafe { &*next };
        current = next;
    }
}
