use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::records::r#type::Type;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;

impl TypeArena {
    pub fn add_tv(&mut self, tv: Type) -> TypeId {
        let allocated = self.types.allocate(tv);
        unsafe {
            (*as_mutable_type_id(allocated)).owning_arena = self as *mut TypeArena;
        }
        allocated
    }
}
