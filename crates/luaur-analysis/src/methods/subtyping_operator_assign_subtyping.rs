use crate::records::subtyping::Subtyping;

impl Subtyping {
    pub fn operator_assign(&mut self, _other: &Subtyping) {
        unimplemented!("Subtyping copy assignment is deleted in C++");
    }
}
