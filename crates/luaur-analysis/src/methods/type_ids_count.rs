use crate::functions::follow_type::follow_type_id;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl TypeIds {
    /// C++ `size_t count(TypeId ty) const`.
    pub fn count(&self, ty: TypeId) -> usize {
        let ty = unsafe { follow_type_id(ty) };
        match self.types.find(&ty) {
            Some(entry) if *entry => 1,
            _ => 0,
        }
    }
}
