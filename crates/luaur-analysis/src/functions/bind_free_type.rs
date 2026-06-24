//! @interface-stub
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::subsumes_scope::subsumes;
use crate::methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type;
use crate::records::free_type::FreeType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn bind_free_type(a: TypeId, b: TypeId) {
    unsafe {
        let af = get_mutable_type_id::<FreeType>(a);
        let bf = get_mutable_type_id::<FreeType>(b);

        LUAU_ASSERT!(!af.is_null() || !bf.is_null());

        if bf.is_null() {
            bind(a, b);
        } else if af.is_null() {
            bind(b, a);
        } else if subsumes((*bf).scope, (*af).scope) {
            bind(a, b);
        } else if subsumes((*af).scope, (*bf).scope) {
            bind(b, a);
        }
    }
}

unsafe fn bind(ty: TypeId, mut bound_to: TypeId) {
    unifiable_bound_type_id_emplace_type_bound_type(&mut *as_mutable_type_id(ty), &mut bound_to);
}
