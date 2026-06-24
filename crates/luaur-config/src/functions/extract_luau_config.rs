use alloc::string::String;

use crate::functions::create_luau_config_from_luau_table::create_luau_config_from_luau_table;
use crate::functions::extract_config::extract_config;
use crate::records::alias_options::AliasOptions;
use crate::records::config::Config;
use crate::records::interrupt_callbacks::InterruptCallbacks;

pub fn extract_luau_config(
    source: &String,
    config: &mut Config,
    alias_options: Option<AliasOptions>,
    callbacks: InterruptCallbacks,
) -> Option<String> {
    let mut error = String::new();
    let Some(config_table) = extract_config(source, &callbacks, &mut error) else {
        return Some(error);
    };

    if !config_table.contains_str("luau") {
        return None;
    }

    let luau_value = config_table
        .find_str("luau")
        .expect("contains checked above");
    let Some(luau_table) = luau_value.get_table() else {
        return Some(String::from(
            "configuration value for key \"luau\" must be a table",
        ));
    };

    create_luau_config_from_luau_table(config, luau_table, alias_options)
}
