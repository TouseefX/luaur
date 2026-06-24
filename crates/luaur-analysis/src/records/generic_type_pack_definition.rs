use crate::type_aliases::type_pack_id::TypePackId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericTypePackDefinition {
    pub(crate) tp: TypePackId,
    pub(crate) defaultValue: Option<TypePackId>,
}
