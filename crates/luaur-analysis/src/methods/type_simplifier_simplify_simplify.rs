use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeSimplifier {
    pub fn simplify_type_id(&mut self, ty: TypeId) -> TypeId {
        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(core::ptr::null_mut());
        self.simplify_type_id_dense_hash_set_type_id(ty, &mut seen)
    }
}
