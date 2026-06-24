use crate::functions::get_type_alt_j::get_type_id;
use crate::records::apply_type_function::ApplyTypeFunction;
use crate::records::extern_type::ExternType;
use crate::records::generic_type::GenericType;
use crate::type_aliases::type_id::TypeId;

impl ApplyTypeFunction {
    pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
        if !unsafe { get_type_id::<GenericType>(ty) }.is_null() {
            return true;
        } else if !unsafe { get_type_id::<ExternType>(ty) }.is_null() {
            return true;
        } else {
            return false;
        }
    }
}
