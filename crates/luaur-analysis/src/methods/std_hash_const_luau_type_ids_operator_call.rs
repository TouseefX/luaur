use crate::records::std_hash_const_luau_type_ids::std_hash_const_luau_type_ids;
use crate::records::type_ids::TypeIds;

impl std_hash_const_luau_type_ids {
    pub fn operator_call(&self, tys: *const TypeIds) -> usize {
        unsafe { (*tys).get_hash() }
    }
}
