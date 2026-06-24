use crate::records::property_type::Property;

impl Property {
    pub fn makeShared(&mut self) {
        if self.write_ty.is_some() {
            self.write_ty = self.read_ty;
        }
    }
}
