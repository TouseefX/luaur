use crate::records::function_type::FunctionType;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

impl FunctionType {
    pub fn generics(&self) -> &Vec<TypeId> {
        &self.generics
    }
}
