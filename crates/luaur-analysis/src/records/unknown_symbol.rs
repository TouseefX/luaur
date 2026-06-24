#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
    Binding,
    Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownSymbol {
    pub(crate) name: alloc::string::String,
    pub(crate) context: Context,
}

impl UnknownSymbol {
    pub const fn new(name: alloc::string::String, context: Context) -> Self {
        Self { name, context }
    }
}

#[allow(non_snake_case)]
impl UnknownSymbol {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn context(&self) -> Context {
        self.context
    }
}

pub use Context as UnknownSymbol_Context;
