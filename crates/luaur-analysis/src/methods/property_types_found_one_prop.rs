use crate::records::property_types::PropertyTypes;

impl PropertyTypes {
    pub fn found_one_prop(&self) -> bool {
        !self.types_of_prop.is_empty()
    }
}
