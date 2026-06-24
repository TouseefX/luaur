#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct BasicDocumentation {
    pub documentation: alloc::string::String,
    pub learn_more_link: alloc::string::String,
    pub code_sample: alloc::string::String,
}
