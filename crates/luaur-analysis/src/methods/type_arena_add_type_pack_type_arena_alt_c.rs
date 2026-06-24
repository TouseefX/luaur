use crate::functions::as_mutable_type_pack_alt_d::as_mutable_type_pack;
use crate::records::type_arena::TypeArena;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeArena {
    pub fn add_type_pack_vector_type_id_optional_type_pack_id(
        &mut self,
        types: Vec<TypeId>,
        tail: Option<TypePackId>,
    ) -> TypePackId {
        let tp = TypePack { head: types, tail };
        let allocated = self.type_packs.allocate(TypePackVar::from(tp));
        unsafe {
            (*as_mutable_type_pack(allocated)).owningArena = self as *mut TypeArena;
        }
        allocated
    }
}
