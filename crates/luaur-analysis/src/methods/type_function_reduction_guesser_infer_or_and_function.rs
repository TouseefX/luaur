use crate::functions::follow_type::follow_type_id;
use crate::records::type_function_inference_result::TypeFunctionInferenceResult;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionReductionGuesser {
    pub fn infer_or_and_function(
        &mut self,
        instance: *const TypeFunctionInstanceType,
    ) -> TypeFunctionInferenceResult {
        LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });

        let mut lhs_ty = unsafe { follow_type_id((&(*instance).type_arguments)[0]) };
        let mut rhs_ty = unsafe { follow_type_id((&(*instance).type_arguments)[1]) };

        if let Some(ty) = self.try_assign_operand_type(lhs_ty) {
            lhs_ty = unsafe { follow_type_id(ty) };
        }
        if let Some(ty) = self.try_assign_operand_type(rhs_ty) {
            rhs_ty = unsafe { follow_type_id(ty) };
        }

        let unknown_ty = unsafe { (*self.builtins).unknownType };
        let boolean_ty = unsafe { (*self.builtins).booleanType };
        let default_and_or_inference = TypeFunctionInferenceResult {
            operand_inference: alloc::vec![unknown_ty, unknown_ty],
            function_result_inference: boolean_ty,
        };

        let lty = self.normalize(lhs_ty);
        let rty = self.normalize(lhs_ty);
        let lhs_truthy = lty.is_truthy();
        let rhs_truthy = rty.is_truthy();

        // If at the end, we still don't have good substitutions, return the default type
        let function_name = unsafe { (*(*instance).function.as_ptr()).name.as_str() };

        if function_name == "or" {
            if self.operand_is_assignable(lhs_ty) && self.operand_is_assignable(rhs_ty) {
                return default_and_or_inference;
            }
            if self.operand_is_assignable(lhs_ty) {
                return TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![unknown_ty, rhs_ty],
                    function_result_inference: rhs_ty,
                };
            }
            if self.operand_is_assignable(rhs_ty) {
                return TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![lhs_ty, unknown_ty],
                    function_result_inference: lhs_ty,
                };
            }
            if lhs_truthy {
                return TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![lhs_ty, rhs_ty],
                    function_result_inference: lhs_ty,
                };
            }
            if rhs_truthy {
                return TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![unknown_ty, rhs_ty],
                    function_result_inference: rhs_ty,
                };
            }
        }

        if function_name == "and" {
            // (mirrors C++ `instance->function->name == "and"`)
            if self.operand_is_assignable(lhs_ty) && self.operand_is_assignable(rhs_ty) {
                return default_and_or_inference;
            }
            if self.operand_is_assignable(lhs_ty) {
                return TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![],
                    function_result_inference: rhs_ty,
                };
            }
            if self.operand_is_assignable(rhs_ty) {
                return TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![],
                    function_result_inference: lhs_ty,
                };
            }
            if lhs_truthy {
                return TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![lhs_ty, rhs_ty],
                    function_result_inference: rhs_ty,
                };
            } else {
                return TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![lhs_ty, rhs_ty],
                    function_result_inference: lhs_ty,
                };
            }
        }

        default_and_or_inference
    }
}
