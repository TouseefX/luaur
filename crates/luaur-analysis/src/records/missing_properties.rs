use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
    Missing,
    Extra,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MissingProperties {
    pub(crate) super_type: TypeId,
    pub(crate) sub_type: TypeId,
    pub(crate) properties: alloc::vec::Vec<alloc::string::String>,
    pub(crate) context: Context,
}

#[allow(non_snake_case)]
impl MissingProperties {
    pub fn superType(&self) -> TypeId {
        self.super_type
    }

    pub fn subType(&self) -> TypeId {
        self.sub_type
    }

    pub fn properties(&self) -> &[alloc::string::String] {
        &self.properties
    }

    pub fn context(&self) -> Context {
        self.context
    }
}

pub use Context as MissingProperties_Context;
