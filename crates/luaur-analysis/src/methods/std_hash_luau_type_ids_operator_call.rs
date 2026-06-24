use crate::records::std_hash_luau_type_ids::std_hash_luau_type_ids;
use crate::records::type_ids::TypeIds;

impl std_hash_luau_type_ids {
    pub fn operator_call(&self, tys: &TypeIds) -> usize {
        tys.get_hash()
    }
}
