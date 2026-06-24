use crate::records::path::Path;
use crate::type_aliases::component::Component;
use alloc::vec::Vec;

impl Path {
    pub fn path_push(&self, component: Component) -> Path {
        let mut joined: Vec<Component> = self.components.clone();
        joined.push(component);
        Path::path_vector_component(joined)
    }
}
