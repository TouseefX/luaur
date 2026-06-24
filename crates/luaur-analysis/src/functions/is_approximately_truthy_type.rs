use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_approximately_falsy_type::is_approximately_falsy_type;
use crate::records::negation_type::NegationType;
use crate::type_aliases::type_id::TypeId;

pub fn is_approximately_truthy_type(ty: TypeId) -> bool {
    let ty = unsafe { follow_type_id(ty) };
    let nt = unsafe { get_type_id::<NegationType>(ty) };
    if !nt.is_null() {
        return is_approximately_falsy_type(unsafe { (*nt).ty });
    }
    false
}
