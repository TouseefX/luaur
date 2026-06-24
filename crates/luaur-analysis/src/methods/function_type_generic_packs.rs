use crate::records::function_type::FunctionType;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl FunctionType {
    pub fn generic_packs(&self) -> &Vec<TypePackId> {
        &self.generic_packs
    }
}
