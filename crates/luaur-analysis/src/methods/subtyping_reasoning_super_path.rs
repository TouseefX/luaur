use crate::records::path::Path;
use crate::records::subtyping_reasoning::SubtypingReasoning;

impl SubtypingReasoning {
    pub fn super_path(&self) -> &Path {
        &self.super_path
    }
}
