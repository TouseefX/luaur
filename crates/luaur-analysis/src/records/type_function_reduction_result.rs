use crate::enums::reduction::Reduction;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct TypeFunctionReductionResult {
    pub result: Option<TypeId>,
    pub reduction_status: Reduction,
    pub blocked_types: alloc::vec::Vec<TypeId>,
    pub blocked_packs: alloc::vec::Vec<TypePackId>,
    pub error: Option<alloc::string::String>,
    pub messages: alloc::vec::Vec<alloc::string::String>,
}
