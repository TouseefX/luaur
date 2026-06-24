use crate::enums::polarity::Polarity;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type::Name;

#[derive(Debug, Clone)]
pub struct GenericType {
    pub index: i32,
    pub level: TypeLevel,
    pub scope: *mut Scope,
    pub name: Name,
    pub explicit_name: bool,
    pub polarity: Polarity,
}

impl Default for GenericType {
    fn default() -> Self {
        Self {
            index: 0,
            level: TypeLevel::default(),
            scope: core::ptr::null_mut(),
            name: Name::default(),
            explicit_name: false,
            polarity: Polarity::Unknown,
        }
    }
}
