use alloc::string::String;
use luaur_common::records::variant::Variant2;

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigTableKey(pub Variant2<String, f64>);

impl Default for ConfigTableKey {
    fn default() -> Self {
        Self(Variant2::V0(String::default()))
    }
}
