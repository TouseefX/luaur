use crate::records::property_type::Property;

impl Property {
    #[inline]
    pub fn isReadWrite(&self) -> bool {
        self.read_ty.is_some() && self.write_ty.is_some()
    }
}
