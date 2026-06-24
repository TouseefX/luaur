use crate::records::r#type::Type;
use crate::type_aliases::type_variant::TypeVariant;

impl Type {
    pub fn operator_assign_type_variant(&mut self, rhs: TypeVariant) -> &mut Self {
        self.ty = rhs;
        self
    }
}
