#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedTypePackInSubtyping {
    pub(crate) tp: crate::type_aliases::type_pack_id::TypePackId,
}
