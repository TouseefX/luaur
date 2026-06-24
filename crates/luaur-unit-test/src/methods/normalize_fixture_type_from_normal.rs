//! @interface-stub
use crate::records::normalize_fixture::NormalizeFixture;
use luaur_analysis::records::normalized_type::NormalizedType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl NormalizeFixture {
    pub fn type_from_normal(&mut self, norm: &NormalizedType) -> TypeId {
        self.get_frontend();
        self.normalizer
            .as_mut()
            .expect("NormalizeFixture normalizer")
            .type_from_normal(norm)
    }
}
