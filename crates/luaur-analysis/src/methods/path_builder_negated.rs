use crate::enums::type_field::TypeField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderNegated {
    fn negated(&mut self) -> &mut Self;
}

impl PathBuilderNegated for PathBuilder {
    fn negated(&mut self) -> &mut Self {
        self.components
            .push(Component::TypeField(TypeField::Negated));
        self
    }
}
