//! `TypeFunctionReducer::stepType` (TypeFunction.cpp:565-622).

use crate::enums::skip_test_result::SkipTestResult;
use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::follow_type::follow;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::records::unscoped_generic_finder::UnscopedGenericFinder;

impl TypeFunctionReducer {
    pub fn step_type(&mut self) {
        let subject = unsafe { follow(*self.queued_tys.front()) };
        self.queued_tys.pop_front();

        if self
            .irreducible
            .contains(&(subject as *const core::ffi::c_void))
        {
            return;
        }

        let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(subject) };
        if !tfit.is_null() {
            // tfit->function->name == "user"
            let is_user = unsafe { (*tfit).function.as_ref().name == "user" };
            if is_user {
                let mut finder = UnscopedGenericFinder::unscoped_generic_finder();
                finder.traverse_type_id(subject);

                if finder.found_unscoped {
                    // Do not step into this type again
                    self.irreducible.insert(subject as *const core::ffi::c_void);

                    // Let the caller know this type will not become reducible
                    self.result.irreducible_types.insert(subject);

                    if self.get_state_type_id(subject) == TypeFunctionInstanceState::Unsolved {
                        self.set_state_type_id_type_function_instance_state(
                            subject,
                            TypeFunctionInstanceState::Solved,
                        );
                    }

                    return;
                }
            }

            let test_cyclic = self.test_for_skippability_type_id(subject);

            if !self.test_parameters(subject, unsafe { &*tfit })
                && test_cyclic != SkipTestResult::CyclicTypeFunction
            {
                let state = unsafe { (*tfit).state };
                if state == TypeFunctionInstanceState::Stuck
                    || state == TypeFunctionInstanceState::Solved
                {
                    self.try_guessing(subject);
                }

                return;
            }

            if self.try_guessing(subject) {
                return;
            }

            unsafe {
                (*self.ctx.as_ptr()).user_func_name = (*tfit).user_func_name;
            }

            let result: TypeFunctionReductionResult = unsafe {
                let reducer = (*tfit).function.as_ref().reducer;
                let type_arguments = (*tfit).type_arguments.clone();
                let pack_arguments = (*tfit).pack_arguments.clone();
                reducer(subject, type_arguments, pack_arguments, self.ctx.as_ptr())
            };
            self.handle_type_function_reduction_type_id(subject, result);
        }
    }
}
