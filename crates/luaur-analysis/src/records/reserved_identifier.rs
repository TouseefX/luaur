use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReservedIdentifier {
    pub(crate) name: String,
}

impl ReservedIdentifier {
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}

#[allow(non_snake_case)]
impl ReservedIdentifier {
    pub fn name(&self) -> &str {
        &self.name
    }
}
