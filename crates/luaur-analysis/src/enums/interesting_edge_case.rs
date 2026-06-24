#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum InterestingEdgeCase {
    None,
    MetatableCall,
    Intersection,
}

impl Default for InterestingEdgeCase {
    fn default() -> Self {
        Self::None
    }
}
