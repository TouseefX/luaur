use crate::functions::get_approximate_return_type_for_function_call_type_utils::get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn get_approximate_return_type_for_function_call_type_id(ty: TypeId) -> Option<TypePackId> {
    let mut seen = DenseHashSet::new(core::ptr::null());
    get_approximate_return_type_for_function_call_type_id_dense_hash_set_type_id(ty, &mut seen)
}
