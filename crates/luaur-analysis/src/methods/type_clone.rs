use crate::records::r#type::Type;

impl Type {
    pub fn clone(&self) -> Type {
        Type {
            ty: self.ty.clone(),
            persistent: self.persistent,
            documentation_symbol: self.documentation_symbol.clone(),
            owning_arena: self.owning_arena,
        }
    }
}
