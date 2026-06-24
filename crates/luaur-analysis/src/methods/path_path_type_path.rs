use crate::records::path::Path;

impl Path {
    pub fn path() -> Self {
        Self {
            components: alloc::vec::Vec::new(),
        }
    }
}
