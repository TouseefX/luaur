use crate::records::resetter::Resetter;

impl Drop for Resetter {
    fn drop(&mut self) {
        unsafe {
            *self.variance = self.old_value.clone();
        }
    }
}

#[allow(non_snake_case)]
pub fn resetter_resetter() {
    // Kept as a no-op to match the scheduled method symbol name; actual logic is in Drop.
}
