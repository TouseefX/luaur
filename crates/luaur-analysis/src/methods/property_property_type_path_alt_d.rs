use crate::records::property_type_path::Property;

impl Property {
    pub fn property_string(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
