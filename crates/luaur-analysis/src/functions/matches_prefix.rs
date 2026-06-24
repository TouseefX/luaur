use crate::records::path::Path;

pub fn matches_prefix(prefix: &Path, full: &Path) -> bool {
    if prefix.components.len() > full.components.len() {
        return false;
    }

    for i in 0..prefix.components.len() {
        if prefix.components[i] != full.components[i] {
            return false;
        }
    }

    true
}
