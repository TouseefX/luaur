use crate::enums::path_type::PathType;
use crate::functions::extract_alias::extract_alias;
use crate::functions::get_path_type::get_path_type;
use crate::records::alias_cycle_tracker::AliasCycleTracker;
use crate::records::navigator::Navigator;
use crate::type_aliases::error_require_navigator::Error;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_config::records::config::Config;

impl Navigator {
    pub fn navigate_impl(&mut self, path: &str) -> Error {
        let path_type = get_path_type(path);

        if path_type == PathType::Unsupported {
            return Some(String::from(
                "require path must start with a valid prefix: ./, ../, or @",
            ));
        }

        if path_type == PathType::Aliased {
            let mut alias = extract_alias(path);
            alias.make_ascii_lowercase();

            if luaur_common::DFFlag::LuauRequireAliasOverrideOrderFix.get() {
                if let Some(error) = self.reset_to_requirer() {
                    return Some(error);
                }
            }

            let (error, was_overridden) = self.to_alias_override(&alias);
            if error.is_some() {
                return error;
            } else if was_overridden {
                if let Some(error) = self.navigate_through_path(path) {
                    return Some(error);
                }

                return None;
            }

            if !luaur_common::DFFlag::LuauRequireAliasOverrideOrderFix.get() {
                if let Some(error) = self.reset_to_requirer() {
                    return Some(error);
                }
            }

            let mut config = Config::default();
            if let Some(error) = self.navigate_to_and_populate_config(&alias, &mut config) {
                return Some(error);
            }

            if config.aliases.contains(&alias) {
                let cycle_tracker = AliasCycleTracker {
                    seen: DenseHashSet::new(String::new()),
                    ordered: Vec::new(),
                };
                if let Some(error) = self.navigate_to_alias(&alias, &config, cycle_tracker) {
                    return Some(error);
                }
                if let Some(error) = self.navigate_through_path(path) {
                    return Some(error);
                }

                return None;
            }

            if alias == "self" {
                if let Some(error) = self.reset_to_requirer() {
                    return Some(error);
                }
                if let Some(error) = self.navigate_through_path(path) {
                    return Some(error);
                }

                return None;
            }

            if let Some(error) = self.to_alias_fallback(&alias) {
                return Some(error);
            }
            if let Some(error) = self.navigate_through_path(path) {
                return Some(error);
            }

            return None;
        }

        if path_type == PathType::RelativeToCurrent || path_type == PathType::RelativeToParent {
            if let Some(error) = self.reset_to_requirer() {
                return Some(error);
            }
            if let Some(error) = self.navigate_to_parent(None) {
                return Some(error);
            }
            if let Some(error) = self.navigate_through_path(path) {
                return Some(error);
            }
        }

        None
    }
}
