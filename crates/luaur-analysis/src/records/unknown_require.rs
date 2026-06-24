use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownRequire {
    pub(crate) module_path: String,
}

impl UnknownRequire {
    pub const fn new(module_path: String) -> Self {
        Self { module_path }
    }
}

#[allow(non_snake_case)]
impl UnknownRequire {
    pub fn modulePath(&self) -> &str {
        &self.module_path
    }
}
