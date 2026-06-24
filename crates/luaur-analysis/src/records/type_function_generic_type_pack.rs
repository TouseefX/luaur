use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFunctionGenericTypePack {
    pub(crate) is_named: bool,
    pub(crate) name: String,
}

impl Default for TypeFunctionGenericTypePack {
    fn default() -> Self {
        Self {
            is_named: false,
            name: String::new(),
        }
    }
}

#[allow(non_snake_case)]
impl TypeFunctionGenericTypePack {
    pub fn isNamed(&self) -> bool {
        self.is_named
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
