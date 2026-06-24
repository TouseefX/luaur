use crate::functions::is_valid_alias::isValidAlias;
use crate::records::alias_options::AliasOptions;
use crate::records::config::Config;
use crate::type_aliases::error::Error;

pub fn parse_alias(
    config: &mut Config,
    alias_key: &str,
    alias_value: &str,
    alias_options: &Option<AliasOptions>,
) -> Error {
    if !isValidAlias(alias_key) {
        return Some(alloc::format!("Invalid alias {}", alias_key));
    }

    let Some(options) = alias_options else {
        return Some(alloc::string::String::from(
            "Cannot parse aliases without alias options",
        ));
    };

    // DenseHashMap<String, ...> requires &String for .contains()
    let alias_key_string = alloc::string::String::from(alias_key);

    if options.overwrite_aliases || !config.aliases.contains(&alias_key_string) {
        if let Some(config_location) = &options.config_location {
            config.set_alias(
                alias_key_string,
                alloc::string::String::from(alias_value),
                config_location,
            );
        } else {
            config.set_alias_simple(alias_key_string, alloc::string::String::from(alias_value));
        }
    }

    None
}
