use crate::functions::as_mutable_type_pack_alt_d::as_mutable_type_pack;
use crate::records::type_arena::TypeArena;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeArena {
    pub fn add_type_pack_type_pack_var(&mut self, tp: TypePackVar) -> TypePackId {
        let allocated = self.type_packs.allocate(tp);
        unsafe {
            (*as_mutable_type_pack(allocated)).owningArena = self as *mut TypeArena;
        }
        allocated
    }
}
