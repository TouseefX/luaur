use crate::enums::unify_result::UnifyResult;
use crate::records::any_type::AnyType;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::function_type::FunctionType;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_pack_id::TypePackId;

impl Unifier2 {
    pub fn unify_any_type_function_type(
        &mut self,
        _sub_any: &AnyType,
        super_fn: &FunctionType,
    ) -> UnifyResult {
        let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
        let arg_result =
            self.unify_type_pack_id_type_pack_id(super_fn.arg_types, builtin_types.anyTypePack);
        let ret_result =
            self.unify_type_pack_id_type_pack_id(builtin_types.anyTypePack, super_fn.ret_types);
        arg_result & ret_result
    }
}
