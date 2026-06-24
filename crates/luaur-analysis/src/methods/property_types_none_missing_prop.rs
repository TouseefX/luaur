use crate::records::property_types::PropertyTypes;

impl PropertyTypes {
    pub fn none_missing_prop(&self) -> bool {
        self.missing_prop.is_empty()
    }
}
