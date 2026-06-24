use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalizer::Normalizer;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn intersect_functions(
        &mut self,
        heres: &mut NormalizedFunctionType,
        theres: &NormalizedFunctionType,
    ) {
        self.consume_fuel();

        if heres.is_never() {
            return;
        } else if theres.is_never() {
            heres.reset_to_never();
            return;
        } else {
            for there in theres.parts.order.iter() {
                let there = *there;
                self.intersect_functions_with_function(heres, there);
            }
        }
    }
}
