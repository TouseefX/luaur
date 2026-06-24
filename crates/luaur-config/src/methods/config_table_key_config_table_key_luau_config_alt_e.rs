use crate::records::config_table_key::ConfigTableKey;

impl ConfigTableKey {
    pub fn config_table_key_f64(d: f64) -> Self {
        ConfigTableKey::config_table_key_string(d.to_string())
    }
}
