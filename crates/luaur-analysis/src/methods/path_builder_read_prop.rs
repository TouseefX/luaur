use crate::records::path_builder::PathBuilder;
use crate::records::property_type_path::Property;
use crate::type_aliases::component::Component;

impl PathBuilder {
    pub fn read_prop(&mut self, name: &str) -> &mut Self {
        self.components
            .push(Component::Property(Property::property_string_bool(
                name, true,
            )));
        self
    }
}
