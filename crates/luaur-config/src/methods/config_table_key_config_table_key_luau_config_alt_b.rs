use alloc::string::String;

use crate::records::config_table_key::ConfigTableKey;

impl ConfigTableKey {
    pub fn config_table_key_string_mut(s: &String) -> Self {
        ConfigTableKey::config_table_key_string(s.clone())
    }
}
