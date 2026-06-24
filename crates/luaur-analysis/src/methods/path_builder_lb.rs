use crate::enums::type_field::TypeField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderLb {
    fn lb(&mut self) -> &mut Self;
}

impl PathBuilderLb for PathBuilder {
    fn lb(&mut self) -> &mut Self {
        self.components
            .push(Component::TypeField(TypeField::LowerBound));
        self
    }
}
