use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type::GenericType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type::Name;

impl GenericType {
    pub fn generic_type() -> Self {
        let index = fresh_index();
        let name = Name::from(format!("g{}", index).as_str());
        GenericType {
            index,
            level: TypeLevel::default(),
            scope: core::ptr::null_mut(),
            name,
            explicit_name: false,
            polarity: Polarity::Unknown,
        }
    }
}
