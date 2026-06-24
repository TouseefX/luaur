use crate::records::r#type::Type;

impl Type {
    #[allow(non_snake_case)]
    pub fn operator_assign_type_item(&mut self, rhs: Type) -> &mut Self {
        self.ty = rhs.ty;
        self.persistent = rhs.persistent;
        self.documentation_symbol = rhs.documentation_symbol;
        self.owning_arena = rhs.owning_arena;
        self
    }
}
