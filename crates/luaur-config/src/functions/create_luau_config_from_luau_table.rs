use alloc::string::String;
use alloc::vec::Vec;

use luaur_common::records::variant::Variant2;

use crate::functions::parse_alias::parse_alias;
use crate::functions::parse_lint_rule_string::parse_lint_rule_string;
use crate::functions::parse_mode_string::parse_mode_string;
use crate::records::alias_options::AliasOptions;
use crate::records::config::Config;
use crate::records::config_table::ConfigTable;
use crate::records::config_table_key::ConfigTableKey;

pub fn create_luau_config_from_luau_table(
    config: &mut Config,
    luau_table: &ConfigTable,
    alias_options: Option<AliasOptions>,
) -> Option<String> {
    for (k, v) in luau_table.iter() {
        let Some(key) = key_string(k) else {
            return Some(String::from(
                "configuration keys in \"luau\" table must be strings",
            ));
        };

        if key == "languagemode" {
            let Some(value) = v.get_string() else {
                return Some(String::from(
                    "configuration value for key \"languagemode\" must be a string",
                ));
            };

            if let Some(error_message) = parse_mode_string(&mut config.mode, value, false) {
                return Some(error_message);
            }
        }

        if key == "lint" {
            let Some(lint) = v.get_table() else {
                return Some(String::from(
                    "configuration value for key \"lint\" must be a table",
                ));
            };

            if let Some(value) = lint.find_str("*") {
                let Some(enabled) = value.get_bool() else {
                    return Some(String::from(
                        "configuration values in \"lint\" table must be booleans",
                    ));
                };

                if let Some(error_message) = parse_lint_rule_string(
                    &mut config.enabled_lint,
                    &mut config.fatal_lint,
                    "*",
                    if *enabled { "true" } else { "false" },
                    false,
                ) {
                    return Some(error_message);
                }
            }

            for (k, v) in lint.iter() {
                let Some(warning_name) = key_string(k) else {
                    return Some(String::from(
                        "configuration keys in \"lint\" table must be strings",
                    ));
                };

                if warning_name == "*" {
                    continue;
                }

                let Some(enabled) = v.get_bool() else {
                    return Some(String::from(
                        "configuration values in \"lint\" table must be booleans",
                    ));
                };

                if let Some(error_message) = parse_lint_rule_string(
                    &mut config.enabled_lint,
                    &mut config.fatal_lint,
                    warning_name,
                    if *enabled { "true" } else { "false" },
                    false,
                ) {
                    return Some(error_message);
                }
            }
        }

        if key == "linterrors" {
            let Some(value) = v.get_bool() else {
                return Some(String::from(
                    "configuration value for key \"linterrors\" must be a boolean",
                ));
            };

            config.lint_errors = *value;
        }

        if key == "typeerrors" {
            let Some(value) = v.get_bool() else {
                return Some(String::from(
                    "configuration value for key \"typeerrors\" must be a boolean",
                ));
            };

            config.type_errors = *value;
        }

        if key == "globals" {
            let Some(globals_table) = v.get_table() else {
                return Some(String::from(
                    "configuration value for key \"globals\" must be an array of strings",
                ));
            };

            let mut globals = Vec::new();
            globals.resize(globals_table.size(), String::new());

            for (k, v) in globals_table.iter() {
                let Some(key) = key_number(k) else {
                    return Some(String::from(
                        "configuration array \"globals\" must only have numeric keys",
                    ));
                };

                let index = *key as usize;
                if index < 1 || globals_table.size() < index {
                    return Some(String::from(
                        "configuration array \"globals\" contains invalid numeric key",
                    ));
                }

                let Some(global) = v.get_string() else {
                    return Some(String::from(
                        "configuration value in \"globals\" table must be a string",
                    ));
                };

                globals[index - 1] = global.clone();
            }

            config.globals = globals;
        }

        if key == "aliases" {
            let Some(aliases) = v.get_table() else {
                return Some(String::from(
                    "configuration value for key \"aliases\" must be a table",
                ));
            };

            for (k, v) in aliases.iter() {
                let Some(alias_key) = key_string(k) else {
                    return Some(String::from(
                        "configuration keys in \"aliases\" table must be strings",
                    ));
                };

                let Some(alias_value) = v.get_string() else {
                    return Some(String::from(
                        "configuration values in \"aliases\" table must be strings",
                    ));
                };

                if let Some(error_message) =
                    parse_alias(config, alias_key, alias_value, &alias_options)
                {
                    return Some(error_message);
                }
            }
        }
    }

    None
}

fn key_string(key: &ConfigTableKey) -> Option<&String> {
    match &key.0 {
        Variant2::V0(value) => Some(value),
        Variant2::V1(_) => None,
    }
}

fn key_number(key: &ConfigTableKey) -> Option<&f64> {
    match &key.0 {
        Variant2::V0(_) => None,
        Variant2::V1(value) => Some(value),
    }
}
