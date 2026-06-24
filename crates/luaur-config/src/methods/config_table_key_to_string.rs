use alloc::string::String;
use alloc::string::ToString;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;
use luaur_common::records::variant::Variant2;

use crate::records::config_table_key::ConfigTableKey;

impl ConfigTableKey {
    pub fn to_string(&self) -> String {
        match &self.0 {
            Variant2::V0(str) => str.clone(),
            Variant2::V1(number) => number.to_string(),
            _ => {
                LUAU_UNREACHABLE!();
            }
        }
    }
}
