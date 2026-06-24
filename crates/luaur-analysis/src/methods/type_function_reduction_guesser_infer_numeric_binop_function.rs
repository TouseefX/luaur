use crate::records::type_function_inference_result::TypeFunctionInferenceResult;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionReductionGuesser {
    pub fn infer_numeric_binop_function(
        &mut self,
        instance: *const TypeFunctionInstanceType,
    ) -> TypeFunctionInferenceResult {
        LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });

        unsafe {
            let builtins = self.builtins;
            TypeFunctionInferenceResult {
                operand_inference: alloc::vec![(*builtins).numberType, (*builtins).numberType,],
                function_result_inference: (*builtins).numberType,
            }
        }
    }
}
