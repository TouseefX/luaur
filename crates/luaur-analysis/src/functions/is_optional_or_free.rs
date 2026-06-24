use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_optional::is_optional;
use crate::records::free_type::FreeType;
use crate::type_aliases::type_id::TypeId;

pub fn is_optional_or_free(ty: TypeId) -> bool {
    let followed = unsafe { follow_type_id(ty) };
    unsafe { is_optional(followed) || !get_type_id::<FreeType>(followed).is_null() }
}
