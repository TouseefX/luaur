use crate::enums::unify_result::UnifyResult;
use crate::records::any_type::AnyType;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::function_type::FunctionType;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_pack_id::TypePackId;

impl Unifier2 {
    pub fn unify_function_type_any_type(
        &mut self,
        sub_fn: &FunctionType,
        _super_any: &AnyType,
    ) -> UnifyResult {
        let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
        let arg_result =
            self.unify_type_pack_id_type_pack_id(builtin_types.anyTypePack, sub_fn.arg_types);
        let ret_result =
            self.unify_type_pack_id_type_pack_id(sub_fn.ret_types, builtin_types.anyTypePack);
        arg_result & ret_result
    }
}
