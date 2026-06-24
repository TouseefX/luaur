use crate::records::property_types::PropertyTypes;

impl PropertyTypes {
    pub fn found_missing_prop(&self) -> bool {
        !self.missing_prop.is_empty()
    }
}
