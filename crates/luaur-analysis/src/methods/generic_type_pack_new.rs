use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type_pack::GenericTypePack;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type::Name;

impl GenericTypePack {
    pub fn new() -> Self {
        Self {
            index: fresh_index(),
            level: TypeLevel::default(),
            scope: core::ptr::null_mut(),
            name: Name::default(),
            explicitName: false,
            polarity: Polarity::Unknown,
        }
    }

    pub fn new_name(name: Name) -> Self {
        Self {
            index: fresh_index(),
            level: TypeLevel::default(),
            scope: core::ptr::null_mut(),
            name,
            explicitName: true,
            polarity: Polarity::Unknown,
        }
    }

    pub fn new_scope_name_polarity(scope: *mut Scope, name: Name, polarity: Polarity) -> Self {
        Self {
            index: fresh_index(),
            level: TypeLevel::default(),
            scope,
            name,
            explicitName: true,
            polarity,
        }
    }
}
