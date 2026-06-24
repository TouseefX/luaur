use crate::enums::type_field::TypeField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

impl PathBuilder {
    pub fn index_key(&mut self) -> &mut Self {
        self.components
            .push(Component::TypeField(TypeField::IndexLookup));
        self
    }
}
