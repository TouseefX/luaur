use crate::enums::pack_field::PackField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderArgs {
    fn args(&mut self) -> &mut Self;
}

impl PathBuilderArgs for PathBuilder {
    fn args(&mut self) -> &mut Self {
        self.components
            .push(Component::PackField(PackField::Arguments));
        self
    }
}
