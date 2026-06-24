use crate::functions::parse_alias::parse_alias;
use crate::functions::parse_boolean::parse_boolean;
use crate::functions::parse_json::parse_json;
use crate::functions::parse_lint_rule_string::parse_lint_rule_string;
use crate::functions::parse_mode_string::parse_mode_string;
use crate::records::config::Config;
use crate::records::config_options::ConfigOptions;
use crate::type_aliases::error::Error;

pub fn parse_config(contents: &str, config: &mut Config, options: &ConfigOptions) -> Error {
    parse_json(contents, |keys: &Vec<String>, value: String| {
        if keys.len() == 1 && keys[0] == "languageMode" {
            return parse_mode_string(&mut config.mode, &value, options.compat);
        } else if keys.len() == 2 && keys[0] == "lint" {
            return parse_lint_rule_string(
                &mut config.enabled_lint,
                &mut config.fatal_lint,
                &keys[1],
                &value,
                options.compat,
            );
        } else if keys.len() == 1 && keys[0] == "lintErrors" {
            return parse_boolean(&mut config.lint_errors, &value);
        } else if keys.len() == 1 && keys[0] == "typeErrors" {
            return parse_boolean(&mut config.type_errors, &value);
        } else if keys.len() == 1 && keys[0] == "globals" {
            config.globals.push(value);
            return None;
        } else if keys.len() == 2 && keys[0] == "aliases" {
            return parse_alias(config, &keys[1], &value, &options.alias_options);
        } else if options.compat && keys.len() == 2 && keys[0] == "language" && keys[1] == "mode" {
            return parse_mode_string(&mut config.mode, &value, options.compat);
        }

        let mut keysv = Vec::<&str>::with_capacity(keys.len());
        for k in keys {
            keysv.push(k.as_str());
        }
        Some(alloc::format!("Unknown key {}", keysv.join("/")))
    })
}
