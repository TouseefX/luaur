use crate::records::normalized_string_type::NormalizedStringType;

impl NormalizedStringType {
    pub fn is_never(&self) -> bool {
        !self.isCofinite && self.singletons.is_empty()
    }
}
