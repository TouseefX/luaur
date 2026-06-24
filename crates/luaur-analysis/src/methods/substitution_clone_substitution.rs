use crate::functions::shallow_clone_substitution::shallow_clone_type_id_type_arena_txn_log;
use crate::records::substitution::Substitution;
use crate::type_aliases::type_id::TypeId;

impl Substitution {
    pub fn clone_type_id(&mut self, ty: TypeId) -> TypeId {
        let arena = unsafe { &mut *self.arena };
        shallow_clone_type_id_type_arena_txn_log(ty, arena, self.base.log)
    }
}
