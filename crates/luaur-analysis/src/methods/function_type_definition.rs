use crate::records::function_definition::FunctionDefinition;
use crate::records::function_type::FunctionType;

impl FunctionType {
    pub fn definition(&self) -> Option<&FunctionDefinition> {
        self.definition.as_ref()
    }
}
