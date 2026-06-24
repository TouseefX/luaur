use crate::functions::follow_type::follow_type_id;
use crate::records::type_function_inference_result::TypeFunctionInferenceResult;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionReductionGuesser {
    pub fn infer_comparison_function(
        &mut self,
        instance: *const TypeFunctionInstanceType,
    ) -> TypeFunctionInferenceResult {
        LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });
        // Comparison functions are lt/le/eq.
        // Heuristic: these are type functions from t -> t -> bool

        let mut lhs_ty = unsafe { follow_type_id((&(*instance).type_arguments)[0]) };
        let mut rhs_ty = unsafe { follow_type_id((&(*instance).type_arguments)[1]) };

        let boolean_ty = unsafe { (*self.builtins).booleanType };
        let comparison_inference = |op: TypeId| -> TypeFunctionInferenceResult {
            TypeFunctionInferenceResult {
                operand_inference: alloc::vec![op, op],
                function_result_inference: boolean_ty,
            }
        };

        if let Some(ty) = self.try_assign_operand_type(lhs_ty) {
            lhs_ty = unsafe { follow_type_id(ty) };
        }
        if let Some(ty) = self.try_assign_operand_type(rhs_ty) {
            rhs_ty = unsafe { follow_type_id(ty) };
        }
        if self.operand_is_assignable(lhs_ty) && !self.operand_is_assignable(rhs_ty) {
            return comparison_inference(rhs_ty);
        }
        if self.operand_is_assignable(rhs_ty) && !self.operand_is_assignable(lhs_ty) {
            return comparison_inference(lhs_ty);
        }
        comparison_inference(unsafe { (*self.builtins).numberType })
    }
}
