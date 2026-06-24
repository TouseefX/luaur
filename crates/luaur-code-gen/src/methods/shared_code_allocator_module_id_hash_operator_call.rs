use crate::records::module_id_hash::ModuleIdHash;
use crate::type_aliases::module_id::ModuleId;
use core::hash::{Hash, Hasher};

impl ModuleIdHash {
    #[allow(non_snake_case)]
    #[inline]
    pub fn shared_code_allocator_module_id_hash_operator_call(
        &self,
        module_id: &ModuleId,
    ) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        module_id.hash(&mut hasher);
        hasher.finish() as usize
    }
}
