use crate::records::property_type_path::Property;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

pub fn make_property(ty: TypeId, documentation_symbol: Option<String>) -> Property {
    Property {
        name: documentation_symbol.unwrap_or_default(),
        is_read: true,
    }
}
