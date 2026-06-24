use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type::GenericType;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type::Name;

impl GenericType {
    pub fn generic_type_scope_name(scope: *mut Scope, name: &Name) -> Self {
        GenericType {
            index: fresh_index(),
            level: TypeLevel::default(),
            scope,
            name: name.clone(),
            explicit_name: true,
            polarity: Polarity::Unknown,
        }
    }
}
