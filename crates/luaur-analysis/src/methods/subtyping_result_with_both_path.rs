use crate::records::subtyping_result::SubtypingResult;
use crate::type_aliases::path::Path;

impl SubtypingResult {
    pub fn with_both_path(&mut self, path: Path) -> &mut Self {
        self.with_sub_path(path.clone()).with_super_path(path)
    }
}
