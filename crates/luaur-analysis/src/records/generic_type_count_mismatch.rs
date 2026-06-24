#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericTypeCountMismatch {
    pub(crate) sub_ty_generic_count: usize,
    pub(crate) super_ty_generic_count: usize,
}

impl Default for GenericTypeCountMismatch {
    fn default() -> Self {
        Self {
            sub_ty_generic_count: 0,
            super_ty_generic_count: 0,
        }
    }
}

impl GenericTypeCountMismatch {
    pub fn sub_ty_generic_count(&self) -> usize {
        self.sub_ty_generic_count
    }

    pub fn super_ty_generic_count(&self) -> usize {
        self.super_ty_generic_count
    }
}
