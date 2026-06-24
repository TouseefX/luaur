use crate::functions::is_cacheable_normalize_alt_b::is_cacheable;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn is_cacheable_type_id(ty: TypeId) -> bool {
    let mut seen = DenseHashSet::new(core::ptr::null());
    is_cacheable(ty, &mut seen)
}
