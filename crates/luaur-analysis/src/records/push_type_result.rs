use crate::records::incomplete_inference::IncompleteInference;

#[derive(Debug, Clone, Default)]
pub struct PushTypeResult {
    pub incomplete_types: alloc::vec::Vec<IncompleteInference>,
}
