use crate::enums::unify_result::UnifyResult;
use crate::records::any_type::AnyType;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::metatable_type::MetatableType;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;

impl Unifier2 {
    pub fn unify_any_type_metatable_type(
        &mut self,
        _sub_any: &AnyType,
        super_metatable: &MetatableType,
    ) -> UnifyResult {
        let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
        let metatable_result =
            self.unify_type_id_type_id(builtin_types.anyType, super_metatable.metatable);
        if metatable_result != UnifyResult::Ok {
            return metatable_result;
        }
        self.unify_type_id_type_id(builtin_types.anyType, super_metatable.table)
    }
}
