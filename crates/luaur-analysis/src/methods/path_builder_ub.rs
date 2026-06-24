use crate::enums::type_field::TypeField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderUb {
    fn ub(&mut self) -> &mut Self;
}

impl PathBuilderUb for PathBuilder {
    fn ub(&mut self) -> &mut Self {
        self.components
            .push(Component::TypeField(TypeField::UpperBound));
        self
    }
}
