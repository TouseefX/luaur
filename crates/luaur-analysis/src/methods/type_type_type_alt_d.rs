use crate::records::r#type::Type;

impl Type {
    pub fn type_item_type_item_mut(&mut self, rhs: Type) {
        self.ty = rhs.ty;
        self.persistent = rhs.persistent;
        self.documentation_symbol = rhs.documentation_symbol;
        self.owning_arena = rhs.owning_arena;
    }
}
