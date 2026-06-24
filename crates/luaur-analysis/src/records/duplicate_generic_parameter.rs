use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuplicateGenericParameter {
    pub(crate) parameterName: String,
}

impl DuplicateGenericParameter {
    pub const fn new(parameter_name: String) -> Self {
        Self {
            parameterName: parameter_name,
        }
    }
}

#[allow(non_snake_case)]
impl DuplicateGenericParameter {
    pub fn parameterName(&self) -> &str {
        &self.parameterName
    }
}
