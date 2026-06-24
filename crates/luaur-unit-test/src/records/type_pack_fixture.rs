#[derive(Debug, Clone)]
pub struct TypePackFixture {
    pub(crate) type_packs:
        alloc::vec::Vec<alloc::boxed::Box<luaur_analysis::records::type_pack_var::TypePackVar>>,
    pub(crate) type_vars: alloc::vec::Vec<alloc::boxed::Box<luaur_analysis::records::r#type::Type>>,
    pub(crate) types: alloc::vec::Vec<luaur_analysis::type_aliases::type_id::TypeId>,
}

impl Default for TypePackFixture {
    fn default() -> Self {
        Self {
            type_packs: alloc::vec::Vec::new(),
            type_vars: alloc::vec::Vec::new(),
            types: alloc::vec::Vec::new(),
        }
    }
}
