use crate::records::std_equal_to_const_luau_type_ids::std_equal_to_const_luau_type_ids;
use crate::records::type_ids::TypeIds;

impl std_equal_to_const_luau_type_ids {
    pub fn operator_call(&self, here: *const TypeIds, there: *const TypeIds) -> bool {
        unsafe { (*here).operator_eq(&*there) }
    }
}
