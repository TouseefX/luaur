use crate::records::index::Index;
use crate::records::path_builder::PathBuilder;
use crate::type_aliases::component::Component;

pub trait PathBuilderIndex {
    fn index(&mut self, i: usize) -> &mut Self;
}

impl PathBuilderIndex for PathBuilder {
    fn index(&mut self, i: usize) -> &mut Self {
        self.components.push(Component::Index(Index {
            index: i,
            variant: crate::enums::variant::Variant::Pack,
        }));
        self
    }
}
