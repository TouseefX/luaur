use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn union_functions(
        &mut self,
        heres: &mut NormalizedFunctionType,
        theres: &NormalizedFunctionType,
    ) {
        self.consume_fuel();

        if heres.is_top {
            return;
        }
        if theres.is_top {
            heres.reset_to_top();
        }

        if theres.is_never() {
            return;
        }

        let mut tmps = TypeIds::type_ids();

        if heres.is_never() {
            tmps = theres.parts.clone();
            heres.parts = tmps;
            return;
        }

        let heres_parts = heres.parts.clone();
        let theres_parts = theres.parts.clone();

        for here in heres_parts.order {
            for there in theres_parts.order.iter() {
                let there = *there;
                if let Some(fun) = self.union_of_functions(here, there) {
                    tmps.insert_type_id(fun);
                } else {
                    let builtin_types = unsafe { &*self.builtin_types };
                    tmps.insert_type_id(builtin_types.error_recovery_type(there));
                }
            }
        }

        heres.parts = tmps;
    }
}
