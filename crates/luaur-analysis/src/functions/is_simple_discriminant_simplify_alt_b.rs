use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn is_simple_discriminant_type_id(ty: TypeId) -> bool {
    let mut seen_set = DenseHashSet::new(core::ptr::null());
    crate::functions::is_simple_discriminant_simplify::is_simple_discriminant(ty, &mut seen_set)
}
