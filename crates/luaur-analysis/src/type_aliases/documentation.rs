use crate::records::basic_documentation::BasicDocumentation;
use crate::records::function_documentation::FunctionDocumentation;
use crate::records::overloaded_function_documentation::OverloadedFunctionDocumentation;
use crate::records::table_documentation::TableDocumentation;
use luaur_common::records::variant::Variant4;

pub type Documentation = Variant4<
    BasicDocumentation,
    FunctionDocumentation,
    TableDocumentation,
    OverloadedFunctionDocumentation,
>;
