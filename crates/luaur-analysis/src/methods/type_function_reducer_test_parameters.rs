use crate::enums::skip_test_result::SkipTestResult;
use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionReducer {
    pub fn test_parameters(&mut self, subject: TypeId, tfit: &TypeFunctionInstanceType) -> bool {
        for p in &tfit.type_arguments {
            let skip = self.test_for_skippability_type_id(*p);

            if skip == SkipTestResult::Stuck {
                self.irreducible.insert(subject as *const core::ffi::c_void);
                self.set_state_type_id_type_function_instance_state(
                    subject,
                    TypeFunctionInstanceState::Stuck,
                );

                return false;
            }

            if skip == SkipTestResult::Irreducible
                || (skip == SkipTestResult::Generic
                    && unsafe { !(*tfit.function.as_ptr()).can_reduce_generics })
            {
                self.irreducible.insert(subject as *const core::ffi::c_void);

                if skip == SkipTestResult::Generic {
                    self.set_state_type_id_type_function_instance_state(
                        subject,
                        TypeFunctionInstanceState::Solved,
                    );
                }

                return false;
            } else if skip == SkipTestResult::Defer {
                self.queued_tys.push_back(subject);
                return false;
            }
        }

        for p in &tfit.pack_arguments {
            let skip = self.test_for_skippability_type_pack_id(*p);

            if skip == SkipTestResult::Irreducible
                || (skip == SkipTestResult::Generic
                    && unsafe { !(*tfit.function.as_ptr()).can_reduce_generics })
            {
                self.irreducible.insert(subject as *const core::ffi::c_void);
                return false;
            } else if skip == SkipTestResult::Defer {
                self.queued_tys.push_back(subject);
                return false;
            }
        }

        true
    }
}
