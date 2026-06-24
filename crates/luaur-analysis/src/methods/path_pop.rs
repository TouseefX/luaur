use crate::records::path::Path;
use crate::type_aliases::component::Component;
use alloc::vec::Vec;

impl Path {
    pub fn path_pop(&self) -> Path {
        if self.components.is_empty() {
            return Path::path();
        }

        let mut popped: Vec<Component> = self.components.clone();
        popped.pop();
        Path::path_vector_component(popped)
    }
}
