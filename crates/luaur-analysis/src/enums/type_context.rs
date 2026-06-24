#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum TypeContext {
    /// the default context
    Default,
    /// inside of a condition
    Condition,
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::Default
    }
}
