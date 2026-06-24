use crate::enums::polarity::Polarity;
use crate::records::generic_type::GenericType;
use crate::records::scope::Scope;

use crate::functions::fresh_index::fresh_index;

impl GenericType {
    pub fn generic_type_scope_polarity(scope: *mut Scope, polarity: Polarity) -> Self {
        GenericType {
            index: fresh_index(),
            level: Default::default(),
            scope,
            name: Default::default(),
            explicit_name: false,
            polarity,
        }
    }
}
