#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FunctionDocumentation {
    pub documentation: alloc::string::String,
    pub parameters: alloc::vec::Vec<
        crate::records::function_parameter_documentation::FunctionParameterDocumentation,
    >,
    pub returns: alloc::vec::Vec<crate::type_aliases::documentation_symbol::DocumentationSymbol>,
    pub learn_more_link: alloc::string::String,
    pub code_sample: alloc::string::String,
}
