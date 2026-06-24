use crate::records::path::Path;
use crate::type_aliases::component::Component;
use alloc::vec::Vec;

impl Path {
    pub fn path_push_front(&self, component: Component) -> Path {
        let mut joined: Vec<Component> = Vec::new();
        joined.reserve(self.components.len() + 1);
        joined.push(component);
        joined.extend_from_slice(&self.components);
        Path::path_vector_component(joined)
    }
}
