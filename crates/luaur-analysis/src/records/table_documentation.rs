#[derive(Debug, Clone)]
pub struct TableDocumentation {
    pub documentation: alloc::string::String,
    pub keys: luaur_common::records::dense_hash_map::DenseHashMap<
        alloc::string::String,
        crate::type_aliases::documentation_symbol::DocumentationSymbol,
    >,
    pub learn_more_link: alloc::string::String,
    pub code_sample: alloc::string::String,
}
