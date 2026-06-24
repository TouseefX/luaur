use crate::records::r#type::Type;
use luaur_common::macros::luau_noinline::LUAU_NOINLINE;

LUAU_NOINLINE! {
    pub fn emplace_type<T>(_ty: *mut Type) -> *mut T {
        panic!("generic emplace_type is not a valid Rust port; use a concrete TypeVariant assignment")
    }
}
