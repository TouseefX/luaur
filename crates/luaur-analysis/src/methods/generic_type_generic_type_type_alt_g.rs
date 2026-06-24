use crate::enums::polarity::Polarity;
use crate::records::generic_type::GenericType;
use crate::records::scope::Scope;
use crate::type_aliases::name_type::Name;

impl GenericType {
    pub fn generic_type_scope_name_polarity(
        scope: *mut Scope,
        name: Name,
        polarity: Polarity,
    ) -> Self {
        GenericType {
            index: crate::functions::fresh_index::fresh_index(),
            level: Default::default(),
            scope,
            name,
            explicit_name: true,
            polarity,
        }
    }
}
