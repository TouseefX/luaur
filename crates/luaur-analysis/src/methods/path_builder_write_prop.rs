use crate::records::path_builder::PathBuilder;
use crate::records::property_type_path::Property;

impl PathBuilder {
    pub fn write_prop(&mut self, name: &str) -> &mut Self {
        self.components
            .push(crate::type_aliases::component::Component::Property(
                Property::property_string_bool(name, false),
            ));
        self
    }
}
