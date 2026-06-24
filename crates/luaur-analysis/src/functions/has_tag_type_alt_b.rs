use crate::functions::follow_type::follow_type_id;
use crate::functions::get_tags::get_tags;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::has_tag_type::has_tag as has_tag_tags;
use crate::records::extern_type::ExternType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn has_tag(ty: TypeId, tag_name: &str) -> bool {
    let ty = unsafe { follow_type_id(ty) };

    unsafe {
        // We special case extern types because getTags only returns a pointer to one vector of tags.
        // But extern types has multiple vector of tags, represented throughout the hierarchy.
        let mut etv = get_type_id::<ExternType>(ty);
        if !etv.is_null() {
            while !etv.is_null() {
                if has_tag_tags(&(*etv).tags, tag_name) {
                    return true;
                } else if (*etv).parent.is_none() {
                    return false;
                }

                etv = get_type_id::<ExternType>((*etv).parent.unwrap());
                LUAU_ASSERT!(!etv.is_null());
            }
        } else if let Some(tags) = get_tags(ty) {
            return has_tag_tags(tags, tag_name);
        }
    }

    false
}
