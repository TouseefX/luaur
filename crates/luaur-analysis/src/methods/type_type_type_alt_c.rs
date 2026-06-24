use crate::records::r#type::Type;
use crate::type_aliases::type_variant::TypeVariant;

impl Type {
    pub fn type_item_type_variant_bool(&mut self, ty: TypeVariant, persistent: bool) {
        self.ty = ty;
        self.persistent = persistent;
    }
}
