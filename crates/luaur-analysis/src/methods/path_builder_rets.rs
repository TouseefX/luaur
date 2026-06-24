use crate::enums::pack_field::PackField;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderRets {
    fn rets(&mut self) -> &mut Self;
}

impl PathBuilderRets for PathBuilder {
    fn rets(&mut self) -> &mut Self {
        self.components
            .push(Component::PackField(PackField::Returns));
        self
    }
}
