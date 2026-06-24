use crate::type_aliases::documentation::Documentation;
use crate::type_aliases::documentation_symbol::DocumentationSymbol;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub type DocumentationDatabase = DenseHashMap<DocumentationSymbol, Documentation>;
