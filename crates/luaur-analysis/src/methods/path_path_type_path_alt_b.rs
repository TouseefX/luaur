use crate::records::path::Path;
use crate::type_aliases::component::Component;

impl Path {
    pub fn path_vector_component(components: alloc::vec::Vec<Component>) -> Self {
        Self { components }
    }
}
