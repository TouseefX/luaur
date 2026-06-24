use crate::functions::make_table_shared_normalize::make_table_shared_type_id_dense_hash_set_type_id;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn make_table_shared_type_id(ty: TypeId) {
    let mut seen = DenseHashSet::new(core::ptr::null());
    make_table_shared_type_id_dense_hash_set_type_id(ty, &mut seen);
}
