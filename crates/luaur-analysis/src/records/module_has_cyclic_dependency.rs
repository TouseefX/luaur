use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleHasCyclicDependency {
    pub(crate) cycle: Vec<String>,
}

impl ModuleHasCyclicDependency {
    pub const fn new(cycle: Vec<String>) -> Self {
        Self { cycle }
    }
}

#[allow(non_snake_case)]
impl ModuleHasCyclicDependency {
    pub fn cycle(&self) -> &[String] {
        &self.cycle
    }
}
