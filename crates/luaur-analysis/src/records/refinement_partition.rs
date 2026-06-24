use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefinementPartition {
    /// Types that we want to intersect against the type of the expression.
    pub(crate) discriminant_types: Vec<TypeId>,
    /// Sometimes the type we're discriminating against is implicitly nil.
    pub(crate) should_append_nil_type: bool,
}

impl Default for RefinementPartition {
    fn default() -> Self {
        Self {
            discriminant_types: Vec::new(),
            should_append_nil_type: false,
        }
    }
}

#[allow(non_snake_case)]
impl RefinementPartition {
    pub fn discriminantTypes(&self) -> &[TypeId] {
        &self.discriminant_types
    }

    pub fn shouldAppendNilType(&self) -> bool {
        self.should_append_nil_type
    }
}
