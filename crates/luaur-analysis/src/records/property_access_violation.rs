use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
    CannotRead,
    CannotWrite,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropertyAccessViolation {
    pub(crate) table: TypeId,
    pub(crate) key: alloc::string::String,
    pub(crate) context: Context,
}

#[allow(non_snake_case)]
impl PropertyAccessViolation {
    pub fn table(&self) -> TypeId {
        self.table
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn context(&self) -> Context {
        self.context
    }
}

pub use Context as PropertyAccessViolation_Context;
