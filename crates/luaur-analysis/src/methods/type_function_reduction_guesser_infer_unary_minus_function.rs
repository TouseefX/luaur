use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_utils::follow_optional_ty;
use crate::functions::is_number::is_number;
use crate::records::type_function_inference_result::TypeFunctionInferenceResult;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionReductionGuesser {
    pub fn infer_unary_minus_function(
        &mut self,
        instance: *const TypeFunctionInstanceType,
    ) -> TypeFunctionInferenceResult {
        unsafe {
            LUAU_ASSERT!((*instance).type_arguments.len() == 1);
            let mut op_ty = follow_type_id((&(*instance).type_arguments)[0]);
            if let Some(ty) = self.try_assign_operand_type(op_ty) {
                op_ty = follow_type_id(ty);
            }
            if is_number(op_ty) {
                TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![op_ty],
                    function_result_inference: (*self.builtins).numberType,
                }
            } else {
                TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![(*self.builtins).unknownType],
                    function_result_inference: (*self.builtins).numberType,
                }
            }
        }
    }
}
