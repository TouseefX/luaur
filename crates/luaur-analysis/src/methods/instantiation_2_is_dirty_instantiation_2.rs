use crate::functions::get_type_alt_j::get_type_id;
use crate::records::generic_type::GenericType;
use crate::records::instantiation_2::Instantiation2;
use crate::type_aliases::type_id::TypeId;

impl Instantiation2 {
    pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
        let gt = unsafe { get_type_id::<GenericType>(ty) };
        if gt.is_null() {
            return false;
        }
        unsafe { self.generic_substitutions.find(&ty) }.is_some()
    }
}
