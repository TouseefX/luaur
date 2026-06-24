//! Source: `Analysis/src/TypePath.cpp:239-243` (hand-ported)
use crate::enums::type_field::TypeField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderVariadic {
    fn variadic(&mut self) -> &mut Self;
}

impl PathBuilderVariadic for PathBuilder {
    fn variadic(&mut self) -> &mut Self {
        self.components
            .push(Component::TypeField(TypeField::Variadic));
        self
    }
}
