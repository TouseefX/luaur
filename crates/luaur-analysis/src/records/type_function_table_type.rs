use crate::records::type_function_property::TypeFunctionProperty;
use crate::records::type_function_table_indexer::TypeFunctionTableIndexer;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use alloc::collections::BTreeMap;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct TypeFunctionTableType {
    pub(crate) props: BTreeMap<String, TypeFunctionProperty>,
    pub(crate) indexer: Option<TypeFunctionTableIndexer>,
    pub(crate) metatable: Option<TypeFunctionTypeId>,
}
