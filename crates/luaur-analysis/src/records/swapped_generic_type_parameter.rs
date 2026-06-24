use crate::enums::kind::Kind;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwappedGenericTypeParameter {
    pub name: String,
    pub kind: Kind,
}

impl SwappedGenericTypeParameter {
    pub const Type: Kind = Kind::Type;
    pub const Pack: Kind = Kind::Pack;
}
