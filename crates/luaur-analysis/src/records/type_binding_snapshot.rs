#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TypeBindingSnapshot {
    pub type_id: alloc::string::String,
    pub type_string: alloc::string::String,
}
