use crate::type_aliases::documentation_symbol::DocumentationSymbol;
use alloc::string::String;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct OverloadedFunctionDocumentation {
    pub overloads: DenseHashMap<String, DocumentationSymbol>,
}
