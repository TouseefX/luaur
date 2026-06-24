use crate::records::config_table_key::ConfigTableKey;

impl ConfigTableKey {
    pub fn config_table_key() -> Self {
        // C++: ConfigTableKey() = default;
        ConfigTableKey::config_table_key_string_mut(&String::new())
    }
}
