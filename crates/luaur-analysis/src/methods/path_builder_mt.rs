use crate::enums::type_field::TypeField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderMt {
    fn mt(&mut self) -> &mut Self;
}

impl PathBuilderMt for PathBuilder {
    fn mt(&mut self) -> &mut Self {
        self.components
            .push(Component::TypeField(TypeField::Metatable));
        self
    }
}
