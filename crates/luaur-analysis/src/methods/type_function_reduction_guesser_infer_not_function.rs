use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_utils::follow_optional_ty;
use crate::records::type_function_inference_result::TypeFunctionInferenceResult;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionReductionGuesser {
    pub fn infer_not_function(
        &mut self,
        instance: *const TypeFunctionInstanceType,
    ) -> TypeFunctionInferenceResult {
        LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 1 });

        let op_ty = unsafe { follow_type_id((&(*instance).type_arguments)[0]) };
        let op_ty = if let Some(ty) = self.try_assign_operand_type(op_ty) {
            unsafe { follow_type_id(ty) }
        } else {
            op_ty
        };

        TypeFunctionInferenceResult {
            operand_inference: alloc::vec![op_ty],
            function_result_inference: unsafe { (*self.builtins).booleanType },
        }
    }
}
