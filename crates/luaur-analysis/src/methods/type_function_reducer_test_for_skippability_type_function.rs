use crate::enums::skip_test_result::SkipTestResult;
use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::functions::follow_type::follow;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::generic_type::GenericType;
use crate::records::intersection_type::IntersectionType;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

impl TypeFunctionReducer {
    pub fn test_for_skippability_type_id(&mut self, ty: TypeId) -> SkipTestResult {
        let mut queue: VecDeque<TypeId> = VecDeque::new();
        let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(TypeId::default());

        queue.push_back(unsafe { follow(ty) });

        while !queue.empty() {
            let t = *queue.front();
            queue.pop_front();

            if seen.contains(&t) {
                continue;
            }

            let tfit = unsafe { get_type_id::<TypeFunctionInstanceType>(t) };
            if !tfit.is_null() {
                let state = unsafe { (*tfit).state };
                if state == TypeFunctionInstanceState::Stuck {
                    return SkipTestResult::Stuck;
                } else if state == TypeFunctionInstanceState::Solved {
                    return SkipTestResult::Generic;
                }

                for cyclic_ty in &self.cyclic_type_functions {
                    if t == *cyclic_ty {
                        return SkipTestResult::CyclicTypeFunction;
                    }
                }

                if !self.irreducible.contains(&(t as *const core::ffi::c_void)) {
                    return SkipTestResult::Defer;
                }

                return SkipTestResult::Irreducible;
            } else if !unsafe { get_type_id::<GenericType>(t) }.is_null() {
                return SkipTestResult::Generic;
            } else {
                let it = unsafe { get_type_id::<IntersectionType>(t) };
                if !it.is_null() {
                    for part in unsafe { &(*it).parts } {
                        queue.push_back(unsafe { follow(*part) });
                    }
                }
            }

            seen.insert(t);
        }

        SkipTestResult::Okay
    }
}
