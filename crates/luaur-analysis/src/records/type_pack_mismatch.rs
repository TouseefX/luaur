#[derive(Debug, Clone, PartialEq)]
pub struct TypePackMismatch {
    pub(crate) wanted_tp: crate::type_aliases::type_pack_id::TypePackId,
    pub(crate) given_tp: crate::type_aliases::type_pack_id::TypePackId,
    pub(crate) reason: alloc::string::String,
}

impl TypePackMismatch {
    pub fn wanted_tp(&self) -> crate::type_aliases::type_pack_id::TypePackId {
        self.wanted_tp
    }

    pub fn given_tp(&self) -> crate::type_aliases::type_pack_id::TypePackId {
        self.given_tp
    }
}
