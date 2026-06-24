use crate::records::normalized_type::NormalizedType;
use crate::records::type_function_reduction_guesser::TypeFunctionReductionGuesser;
use crate::type_aliases::type_id::TypeId;
use std::sync::Arc;

impl TypeFunctionReductionGuesser {
    pub fn normalize(&mut self, ty: TypeId) -> Arc<NormalizedType> {
        let normalizer = self.normalizer;
        unsafe { (*normalizer).normalize(ty) }
    }
}
