use crate::records::type_pack_iterator::TypePackIterator;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypePackIterator {
    pub fn type_pack_iterator() -> Self {
        // TypePackId is currently a stub type; null sentinel deferred until TypePackId = *const TypePackVar
        let null_tp: TypePackId = Default::default();
        Self {
            currentTypePack: null_tp,
            tailCycleCheck: null_tp,
            tp: core::ptr::null(),
            currentIndex: 0,
            log: core::ptr::null(),
        }
    }
}
