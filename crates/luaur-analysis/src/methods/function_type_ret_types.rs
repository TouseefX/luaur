use crate::records::function_type::FunctionType;
use crate::type_aliases::type_pack_id::TypePackId;

impl FunctionType {
    pub fn ret_types(&self) -> TypePackId {
        self.ret_types
    }
}
