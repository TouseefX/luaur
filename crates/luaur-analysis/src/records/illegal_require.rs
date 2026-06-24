use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IllegalRequire {
    pub(crate) moduleName: String,
    pub(crate) reason: String,
}

impl IllegalRequire {
    pub const fn new(module_name: String, reason: String) -> Self {
        Self {
            moduleName: module_name,
            reason,
        }
    }
}

#[allow(non_snake_case)]
impl IllegalRequire {
    pub fn moduleName(&self) -> &str {
        &self.moduleName
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
