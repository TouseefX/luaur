use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;

use crate::records::type_function_inference_result::TypeFunctionInferenceResult;

use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_utils::follow_optional_ty;

use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionReductionGuesser {
    pub fn infer_type_function_substitutions(
        &mut self,
        ty: TypeId,
        instance: *const TypeFunctionInstanceType,
    ) {
        LUAU_ASSERT!(!instance.is_null());

        let result: TypeFunctionInferenceResult;

        unsafe {
            let instance_ref = &*instance;

            if self.is_numeric_binop_function(instance_ref) {
                result = self.infer_numeric_binop_function(instance);
            } else if self.is_comparison_function(instance_ref) {
                result = self.infer_comparison_function(instance);
            } else if self.is_or_and_function(instance_ref) {
                result = self.infer_or_and_function(instance);
            } else if self.is_not_function(instance_ref) {
                result = self.infer_not_function(instance);
            } else if self.is_len_function(instance_ref) {
                result = self.infer_len_function(instance);
            } else if self.is_unary_minus(instance_ref) {
                result = self.infer_unary_minus_function(instance);
            } else {
                result = TypeFunctionInferenceResult {
                    operand_inference: alloc::vec![],
                    function_result_inference: (*self.builtins).unknownType,
                };
            }
        }

        let result_inference = unsafe { follow_type_id(result.function_result_inference) };

        if !self.function_reduces_to.contains(&result_inference) {
            *self.function_reduces_to.get_or_insert(ty) = result_inference;
        }

        unsafe {
            for i in 0..(*instance).type_arguments.len() {
                if i < result.operand_inference.len() {
                    let arg = follow_type_id((&(*instance).type_arguments)[i]);
                    let inference = follow_type_id(result.operand_inference[i]);

                    if !crate::functions::get_type_alt_j::get_type_id::<TypeFunctionInstanceType>(
                        arg,
                    )
                    .is_null()
                    {
                        if !self.function_reduces_to.contains(&arg) {
                            *self.function_reduces_to.get_or_insert(arg) = inference;
                        }
                    } else if !crate::functions::get_type_alt_j::get_type_id::<
                        crate::records::generic_type::GenericType,
                    >(arg)
                    .is_null()
                    {
                        *self.substitutable.get_or_insert(arg) = inference;
                    }
                }
            }
        }
    }
}
