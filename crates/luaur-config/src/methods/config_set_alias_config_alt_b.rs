use alloc::string::String;

use crate::functions::to_lower::to_lower;
use crate::records::config::Config;

impl Config {
    pub fn set_alias_string_string_string(
        &mut self,
        alias: String,
        value: String,
        config_location: &str,
    ) {
        let alias_lower = to_lower(&alias);
        self.set_alias_string_string(alias, value);

        let config_location_string = config_location.to_string();
        if !self
            .config_location_cache
            .contains_key(&config_location_string)
        {
            let owned_location = String::from(config_location);
            let ptr = Box::into_raw(Box::new(owned_location));
            self.config_location_cache
                .try_insert(config_location_string.clone(), ptr);
        }

        let info = self.aliases.get_or_insert(alias_lower);
        let cached_ptr = *self
            .config_location_cache
            .find(&config_location_string)
            .unwrap();
        info.config_location = unsafe { &*cached_ptr }.clone();
    }
}
