use crate::functions::fresh_index::fresh_index;
use crate::records::generic_type::GenericType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::name_type::Name;

impl GenericType {
    pub fn generic_type_type_level_name(_level: TypeLevel, _name: &Name) -> Self {
        let index = fresh_index();
        GenericType {
            index,
            level: _level,
            scope: core::ptr::null_mut(),
            name: _name.clone(),
            explicit_name: true,
            polarity: crate::enums::polarity::Polarity::Unknown,
        }
    }
}
