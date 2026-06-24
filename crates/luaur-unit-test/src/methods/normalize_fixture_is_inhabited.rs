//! @interface-stub
use crate::records::normalize_fixture::NormalizeFixture;
use luaur_analysis::enums::normalization_result::NormalizationResult;
use luaur_analysis::records::normalized_type::NormalizedType;

impl NormalizeFixture {
    pub fn is_inhabited(&mut self, norm: *const NormalizedType) -> bool {
        if norm.is_null() {
            return false;
        }

        self.get_frontend();
        self.normalizer
            .as_mut()
            .expect("NormalizeFixture normalizer")
            .is_inhabited_normalized_type(unsafe { &*norm })
            == NormalizationResult::True
    }
}
