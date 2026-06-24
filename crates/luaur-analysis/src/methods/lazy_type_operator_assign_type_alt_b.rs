use crate::records::lazy_type::LazyType;

impl LazyType {
    pub fn operator_assign_mut(&mut self, rhs: &mut LazyType) -> &mut Self {
        self.unwrap = rhs.unwrap.take();
        self.unwrapped = rhs.unwrapped;
        self
    }
}
