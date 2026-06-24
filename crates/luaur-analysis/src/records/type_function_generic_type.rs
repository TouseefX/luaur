use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFunctionGenericType {
    pub(crate) is_named: bool,
    pub(crate) is_pack: bool,
    pub(crate) name: String,
}

impl Default for TypeFunctionGenericType {
    fn default() -> Self {
        Self {
            is_named: false,
            is_pack: false,
            name: String::new(),
        }
    }
}

#[allow(non_snake_case)]
impl TypeFunctionGenericType {
    pub fn isNamed(&self) -> bool {
        self.is_named
    }

    pub fn isPack(&self) -> bool {
        self.is_pack
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
