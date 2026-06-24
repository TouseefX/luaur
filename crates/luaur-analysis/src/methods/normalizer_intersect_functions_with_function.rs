use crate::functions::get_type_alt_j::get_type_id;
use crate::records::normalized_function_type::NormalizedFunctionType;
use crate::records::normalizer::Normalizer;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;

impl Normalizer {
    pub fn intersect_functions_with_function(
        &mut self,
        heres: &mut NormalizedFunctionType,
        there: TypeId,
    ) {
        self.consume_fuel();

        if heres.is_never() {
            return;
        }

        heres.is_top = false;

        let current_parts = heres.parts.order.clone();
        for here in current_parts {
            let error_ptr = unsafe { get_type_id::<ErrorType>(here) };
            if !error_ptr.is_null() {
                continue;
            }

            if let Some(tmp) = self.intersection_of_functions(here, there) {
                heres.parts.erase_type_id(here);
                heres.parts.insert_type_id(tmp);
                return;
            }
        }

        let mut tmps = TypeIds::type_ids();
        for here in &heres.parts.order {
            if let Some(tmp) = self.union_saturated_functions(*here, there) {
                tmps.insert_type_id(tmp);
            }
        }
        heres.parts.insert_type_id(there);
        for ty in tmps.order {
            heres.parts.insert_type_id(ty);
        }
    }
}
