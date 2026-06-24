use crate::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;
use crate::records::type_fun::TypeFun;

impl IncorrectGenericParameterCount {
    pub fn type_fun(&self) -> &TypeFun {
        &self.type_fun
    }
}
