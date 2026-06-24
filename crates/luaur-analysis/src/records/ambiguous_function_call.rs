#[derive(Debug, Clone, PartialEq)]
pub struct AmbiguousFunctionCall {
    pub(crate) function: crate::type_aliases::type_id::TypeId,
    pub(crate) arguments: crate::type_aliases::type_pack_id::TypePackId,
}
