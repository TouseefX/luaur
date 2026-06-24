use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_alt_e::follow_full;
use crate::functions::is_approximately_falsy_type::is_approximately_falsy_type;
use crate::functions::is_approximately_truthy_type::is_approximately_truthy_type;
use crate::type_aliases::type_id::TypeId;

pub fn is_truthy_or_falsy_type(ty: TypeId) -> bool {
    let ty = unsafe {
        follow_full(
            ty,
            crate::enums::follow_option::FollowOption::Normal,
            core::ptr::null(),
            identity_mapper,
        )
    };
    is_approximately_truthy_type(ty) || is_approximately_falsy_type(ty)
}

fn identity_mapper(_context: *const core::ffi::c_void, t: TypeId) -> TypeId {
    t
}

#[allow(unused_imports)]
pub use is_truthy_or_falsy_type as is_truthy_or_falsy_type_type_id;
