use crate::records::refinement_key::RefinementKey;
use crate::records::typed_allocator::TypedAllocator;

#[derive(Debug)]
pub struct RefinementKeyArena {
    pub(crate) allocator: TypedAllocator<RefinementKey>,
}

impl Default for RefinementKeyArena {
    fn default() -> Self {
        Self {
            allocator: TypedAllocator::default(),
        }
    }
}
