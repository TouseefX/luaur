#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericTypePackCountMismatch {
    pub(crate) subTyGenericPackCount: usize,
    pub(crate) superTyGenericPackCount: usize,
}

impl Default for GenericTypePackCountMismatch {
    fn default() -> Self {
        Self {
            subTyGenericPackCount: 0,
            superTyGenericPackCount: 0,
        }
    }
}

impl GenericTypePackCountMismatch {
    pub fn sub_ty_generic_pack_count(&self) -> usize {
        self.subTyGenericPackCount
    }

    pub fn super_ty_generic_pack_count(&self) -> usize {
        self.superTyGenericPackCount
    }
}
