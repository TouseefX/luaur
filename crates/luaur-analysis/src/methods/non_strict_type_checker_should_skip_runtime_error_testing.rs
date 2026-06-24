use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::type_aliases::type_id::TypeId;

impl NonStrictTypeChecker {
    pub fn should_skip_runtime_error_testing(&mut self, test: TypeId) -> bool {
        let t = unsafe { follow_type_id(test) };
        unsafe {
            !get_type_id::<NeverType>(t).is_null()
                || !get_type_id::<TypeFunctionInstanceType>(t).is_null()
        }
    }
}
