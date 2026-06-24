use crate::records::path::Path;
use crate::type_aliases::component::Component;

impl Path {
    pub fn path_last(&self) -> Option<Component> {
        if self.path_empty() {
            return None;
        }

        Some(self.components[self.components.len() - 1].clone())
    }
}
