use crate::enums::unify_result::UnifyResult;
use crate::records::any_type::AnyType;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::metatable_type::MetatableType;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;

impl Unifier2 {
    pub fn unify_metatable_type_any_type(
        &mut self,
        sub_metatable: &MetatableType,
        _super_any: &AnyType,
    ) -> UnifyResult {
        let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
        let metatable_result =
            self.unify_type_id_type_id(sub_metatable.metatable, builtin_types.anyType);
        if metatable_result != UnifyResult::Ok {
            return metatable_result;
        }
        self.unify_type_id_type_id(sub_metatable.table, builtin_types.anyType)
    }
}
