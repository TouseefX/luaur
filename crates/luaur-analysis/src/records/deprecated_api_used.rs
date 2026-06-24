use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeprecatedApiUsed {
    pub symbol: String,
    pub use_instead: String,
}

impl Default for DeprecatedApiUsed {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            use_instead: String::new(),
        }
    }
}
