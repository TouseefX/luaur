use crate::records::weird_iter::WeirdIter;

impl WeirdIter {
    pub fn weird_iter_weird_iter(&mut self, other: &WeirdIter) {
        *self = other.clone();
    }
}
