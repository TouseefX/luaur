use crate::records::illegal_require::IllegalRequire;

impl IllegalRequire {
    #[inline]
    pub fn operator_eq(&self, rhs: &IllegalRequire) -> bool {
        self.moduleName == rhs.moduleName && self.reason == rhs.reason
    }
}
