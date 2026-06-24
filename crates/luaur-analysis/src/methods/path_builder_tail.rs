use crate::enums::pack_field::PackField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderTail {
    fn tail(&mut self) -> &mut Self;
}

impl PathBuilderTail for PathBuilder {
    fn tail(&mut self) -> &mut Self {
        self.components.push(Component::PackField(PackField::Tail));
        self
    }
}
