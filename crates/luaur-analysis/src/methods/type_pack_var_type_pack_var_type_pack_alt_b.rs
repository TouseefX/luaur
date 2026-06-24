use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_variant::TypePackVariant;

impl TypePackVar {
    pub fn type_pack_var_type_pack_variant_mut(&mut self, tp: TypePackVariant) {
        self.ty = tp;
    }
}
