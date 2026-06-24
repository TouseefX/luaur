use crate::records::type_simplifier::TypeSimplifier;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeSimplifier {
    pub fn intersect_with_simple_discriminant_type_id_type_id(
        &self,
        target: TypeId,
        discriminant: TypeId,
    ) -> Option<TypeId> {
        let mut seen_set = DenseHashSet::new(core::ptr::null());
        self.intersect_with_simple_discriminant_type_id_type_id_dense_hash_set_type_id(
            target,
            discriminant,
            &mut seen_set,
        )
    }
}
