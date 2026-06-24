use crate::records::r#type::Type;
use crate::type_aliases::type_variant::TypeVariant;

impl Type {
    pub fn type_item_type_variant(&mut self, ty: TypeVariant) {
        self.ty = ty;
    }
}
