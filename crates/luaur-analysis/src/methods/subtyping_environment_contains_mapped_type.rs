use crate::functions::follow_type::follow;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::type_aliases::type_id::TypeId;

impl SubtypingEnvironment {
    pub fn contains_mapped_type(&self, ty: TypeId) -> bool {
        let ty = unsafe { follow(ty) };
        if let Some(bounds) = crate::methods::subtyping_bind_generic::dense_hash_map_find_no_default(
            &self.mapped_generics,
            &ty,
        ) {
            if !bounds.is_empty() {
                return true;
            }
        }

        if !self.parent.is_null() {
            return unsafe { (*self.parent).contains_mapped_type(ty) };
        }

        false
    }
}
