use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type::GenericType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type::Name;

impl GenericType {
    pub fn generic_type_name_polarity(name: &Name, polarity: Polarity) -> Self {
        GenericType {
            index: fresh_index(),
            level: TypeLevel::default(),
            scope: core::ptr::null_mut(),
            name: name.clone(),
            explicit_name: true,
            polarity,
        }
    }
}
