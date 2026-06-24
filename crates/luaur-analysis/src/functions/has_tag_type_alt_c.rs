extern crate alloc;

use crate::functions::has_tag_type::has_tag as has_tag_tags;
use crate::records::property_type::Property;

#[allow(non_snake_case)]
pub fn has_tag_property_string(prop: &Property, tag_name: &str) -> bool {
    has_tag_tags(&prop.tags, tag_name)
}
