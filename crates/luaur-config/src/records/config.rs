use alloc::string::String;
use alloc::vec::Vec;

use crate::records::lint_options::LintOptions;
use luaur_ast::enums::mode::Mode;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_table::DenseDefault;

#[derive(Debug)]
pub struct Config {
    pub mode: Mode,
    pub parse_options: ParseOptions,
    pub enabled_lint: LintOptions,
    pub fatal_lint: LintOptions,
    pub lint_errors: bool,
    pub type_errors: bool,
    pub globals: Vec<String>,
    pub aliases: DenseHashMap<String, crate::records::alias_info::AliasInfo>,
    pub config_location_cache: DenseHashMap<String, *mut String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut result = Self {
            mode: Mode::Nonstrict,
            parse_options: ParseOptions::default(),
            enabled_lint: LintOptions::default(),
            fatal_lint: LintOptions::default(),
            lint_errors: false,
            type_errors: true,
            globals: Vec::new(),
            aliases: DenseHashMap::new(String::new()),
            config_location_cache: DenseHashMap::new(String::new()),
        };
        result.enabled_lint.set_defaults();
        result
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        let mut result = Config::default();
        result.config_config_mut(self);
        result
    }
}

impl Config {
    pub fn config_config(&mut self) {
        *self = Config::default();
    }

    pub fn config_config_mut(&mut self, other: &Config) {
        self.mode = other.mode;
        self.parse_options = other.parse_options.clone();
        self.enabled_lint = other.enabled_lint;
        self.fatal_lint = other.fatal_lint;
        self.lint_errors = other.lint_errors;
        self.type_errors = other.type_errors;
        self.globals = other.globals.clone();
        self.aliases = DenseHashMap::new(String::new());
        for (_, ptr) in self.config_location_cache.iter_mut() {
            if !ptr.is_null() {
                unsafe {
                    let _ = Box::from_raw(*ptr);
                }
            }
        }
        self.config_location_cache = DenseHashMap::new(String::new());

        for (_, alias_info) in other.aliases.iter() {
            if alias_info.config_location.is_empty() {
                self.set_alias_string_string(
                    alias_info.original_case.clone(),
                    alias_info.value.clone(),
                );
            } else {
                self.set_alias_string_string_string(
                    alias_info.original_case.clone(),
                    alias_info.value.clone(),
                    &alias_info.config_location,
                );
            }
        }
    }

    pub fn config_assign(&mut self, other: &Config) -> &mut Self {
        self.mode = other.mode;
        self.parse_options = other.parse_options.clone();
        self.enabled_lint = other.enabled_lint;
        self.fatal_lint = other.fatal_lint;
        self.lint_errors = other.lint_errors;
        self.type_errors = other.type_errors;
        self.globals = other.globals.clone();
        self.aliases.clear();
        for (k, v) in other.aliases.iter() {
            let alias_info = crate::records::alias_info::AliasInfo {
                value: v.value.clone(),
                config_location: v.config_location.clone(),
                original_case: v.original_case.clone(),
            };
            self.aliases.try_insert(k.clone(), alias_info);
        }
        for (_, ptr) in self.config_location_cache.iter_mut() {
            if !ptr.is_null() {
                unsafe {
                    let _ = Box::from_raw(*ptr);
                }
            }
        }
        self.config_location_cache.clear();
        for (k, v) in other.config_location_cache.iter() {
            if !v.is_null() {
                let cloned_str = unsafe { (**v).clone() };
                let ptr = Box::into_raw(Box::new(cloned_str));
                self.config_location_cache.try_insert(k.clone(), ptr);
            } else {
                self.config_location_cache
                    .try_insert(k.clone(), core::ptr::null_mut());
            }
        }
        self
    }

    pub fn set_alias(&mut self, alias: String, value: String, config_location: &str) {
        if config_location.is_empty() {
            self.set_alias_string_string(alias, value);
        } else {
            self.set_alias_string_string_string(alias, value, config_location);
        }
    }

    pub fn set_alias_simple(&mut self, alias: String, value: String) {
        self.set_alias_string_string(alias, value);
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        for (_, ptr) in self.config_location_cache.iter_mut() {
            if !ptr.is_null() {
                unsafe {
                    let _ = Box::from_raw(*ptr);
                }
            }
        }
    }
}

impl DenseDefault for crate::records::alias_info::AliasInfo {
    fn dense_default() -> Self {
        Self::default()
    }
}
