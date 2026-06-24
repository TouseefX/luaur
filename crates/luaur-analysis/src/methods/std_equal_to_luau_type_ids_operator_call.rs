use crate::records::std_equal_to_luau_type_ids::std_equal_to_luau_type_ids;
use crate::records::type_ids::TypeIds;

impl std_equal_to_luau_type_ids {
    pub fn operator_call(&self, here: &TypeIds, there: &TypeIds) -> bool {
        here.operator_eq(there)
    }
}
