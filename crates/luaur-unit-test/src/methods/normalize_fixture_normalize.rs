//! @interface-stub
use crate::records::normalize_fixture::NormalizeFixture;
use alloc::sync::Arc;
use luaur_analysis::records::normalized_type::NormalizedType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl NormalizeFixture {
    pub fn normalize(&mut self, ty: TypeId) -> Option<Arc<NormalizedType>> {
        self.get_frontend();
        self.normalizer
            .as_mut()
            .expect("NormalizeFixture normalizer")
            .try_normalize(ty)
    }
}
