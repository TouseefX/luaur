use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_variant::TypePackVariant;

impl TypePackVar {
    pub fn type_pack_var_type_pack_variant_bool(&mut self, tp: TypePackVariant, persistent: bool) {
        self.type_pack_var_type_pack_variant_mut(tp);
        self.persistent = persistent;
    }
}
