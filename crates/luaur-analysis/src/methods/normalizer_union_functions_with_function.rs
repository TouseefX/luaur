use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn union_functions_with_function(
        &mut self,
        heres: &mut NormalizedFunctionType,
        there: TypeId,
    ) {
        self.consume_fuel();

        if heres.is_never() {
            let mut tmps = TypeIds::type_ids();
            tmps.insert_type_id(there);
            heres.parts = tmps;
            return;
        }

        let mut tmps = TypeIds::type_ids();
        let parts = heres.parts.clone();
        for here in parts.order {
            if let Some(fun) = self.union_of_functions(here, there) {
                tmps.insert_type_id(fun);
            } else {
                let builtin_types = unsafe { &*self.builtin_types };
                tmps.insert_type_id(builtin_types.error_recovery_type(there));
            }
        }
        heres.parts = tmps;
    }
}
