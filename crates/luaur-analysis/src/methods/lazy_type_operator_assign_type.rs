use crate::records::lazy_type::LazyType;

impl LazyType {
    pub fn operator_assign(&mut self, rhs: &LazyType) -> &mut Self {
        self.unwrap = rhs.unwrap;
        self.unwrapped = rhs.unwrapped;
        self
    }
}
