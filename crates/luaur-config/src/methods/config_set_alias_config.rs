use alloc::string::String;

use crate::functions::to_lower::to_lower;
use crate::records::alias_info::AliasInfo;
use crate::records::config::Config;

impl Config {
    pub fn set_alias_string_string(&mut self, alias: String, value: String) {
        let alias_lower = to_lower(&alias);
        let info = self.aliases.get_or_insert(alias_lower);
        info.value = value;
        info.original_case = alias;
        info.config_location = String::new();
    }
}
