use crate::functions::get_tags::get_tags;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn attach_tag(ty: TypeId, tag_name: &str) {
    if let Some(tags) = get_tags(ty) {
        tags.push(alloc::string::String::from(tag_name));
    } else {
        LUAU_ASSERT!(false);
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use attach_tag as attach_tag_type_id_string;
