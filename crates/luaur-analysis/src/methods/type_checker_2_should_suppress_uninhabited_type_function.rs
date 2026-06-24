use crate::enums::normalization_result::NormalizationResult;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl TypeChecker2 {
    pub fn should_suppress_uninhabited_type_function_error(&mut self, ty: TypeId) -> bool {
        let ty = unsafe { follow_type_id(ty) };
        let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(ty).as_ref() };
        let Some(tfit) = tfit else {
            return false;
        };

        let function_name = unsafe { (*tfit.function.as_ptr()).name.as_str() };
        let is_numeric = matches!(
            function_name,
            "add" | "sub" | "mul" | "div" | "idiv" | "pow" | "mod"
        );

        if !is_numeric {
            return false;
        }

        for arg in &tfit.type_arguments {
            let arg = unsafe { follow_type_id(*arg) };
            let Some(normalized) = self.normalizer.try_normalize(arg) else {
                continue;
            };

            if self
                .normalizer
                .is_inhabited_normalized_type(normalized.as_ref())
                == NormalizationResult::False
            {
                return true;
            }
        }

        false
    }
}
