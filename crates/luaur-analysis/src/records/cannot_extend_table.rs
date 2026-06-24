use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
    Property,
    Indexer,
    Metatable,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CannotExtendTable {
    pub(crate) table_type: TypeId,
    pub(crate) context: Context,
    pub(crate) prop: alloc::string::String,
}

#[allow(non_snake_case)]
impl CannotExtendTable {
    pub fn tableType(&self) -> TypeId {
        self.table_type
    }

    pub fn context(&self) -> Context {
        self.context
    }

    pub fn prop(&self) -> &str {
        &self.prop
    }
}

pub use Context as CannotExtendTable_Context;
