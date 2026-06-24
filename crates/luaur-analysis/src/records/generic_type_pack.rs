use crate::enums::polarity::Polarity;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type_fwd::Name;

#[derive(Debug, Clone)]
pub struct GenericTypePack {
    pub(crate) index: i32,
    pub(crate) level: TypeLevel,
    pub(crate) scope: *mut Scope,
    pub(crate) name: Name,
    pub(crate) explicitName: bool,
    pub(crate) polarity: Polarity,
}
