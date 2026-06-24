use crate::records::type_arena::TypeArena;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeArena {
    pub fn add_type_pack_t<T>(&mut self, tp: T) -> TypePackId
    where
        T: Into<TypePackVar>,
    {
        self.add_type_pack_type_pack_var(tp.into())
    }
}
