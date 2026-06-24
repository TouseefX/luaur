use crate::enums::variance::Variance;
use crate::records::resetter::Resetter;

impl Resetter {
    pub fn new(variance: *mut Variance) -> Self {
        let old_value = unsafe { *variance };
        Self {
            old_value,
            variance,
        }
    }
}

#[allow(non_snake_case)]
pub fn resetter_resetter(variance: *mut Variance) -> Resetter {
    Resetter::new(variance)
}
