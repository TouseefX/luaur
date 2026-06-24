use crate::records::free_type::FreeType;
use crate::records::quantifier::Quantifier;
use crate::type_aliases::type_id::TypeId;

use crate::functions::as_mutable_type_id::as_mutable_type_id;
use crate::records::generic_type::GenericType;
use crate::records::r#type::Type;

impl Quantifier {
    pub fn visit_type_id_free_type(&mut self, ty: TypeId, ftv: &FreeType) -> bool {
        self.seen_mutable_type = true;

        if !self.level.subsumes(&ftv.level) {
            return false;
        }

        unsafe {
            *as_mutable_type_id(ty) = Type::from(GenericType::generic_type_type_level(self.level));
        }

        self.generics.push(ty);

        false
    }
}
