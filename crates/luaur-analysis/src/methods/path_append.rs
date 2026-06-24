use crate::records::path::Path;
use crate::type_aliases::component::Component;
use alloc::vec::Vec;

impl Path {
    pub fn path_append(&self, suffix: &Path) -> Path {
        let mut joined: Vec<Component> = self.components.clone();
        joined.reserve(suffix.components.len());
        joined.extend_from_slice(&suffix.components);
        Path::path_vector_component(joined)
    }
}
