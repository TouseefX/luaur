use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnsupportedType {
    pub(crate) r#type: TypeId,
}
