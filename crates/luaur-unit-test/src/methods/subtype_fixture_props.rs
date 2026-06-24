use crate::records::subtype_fixture::SubtypeFixture;
use alloc::string::ToString;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::type_aliases::props_type_alt_c::Props;

impl SubtypeFixture {
    pub fn props(entries: Vec<(&str, Property)>) -> Props {
        entries
            .into_iter()
            .map(|(name, property)| (name.to_string(), property))
            .collect()
    }
}
