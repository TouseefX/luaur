use crate::enums::path_type::PathType;
use crate::functions::extract_alias::extract_alias;
use crate::functions::get_path_type::get_path_type;
use crate::records::alias_cycle_tracker::AliasCycleTracker;
use crate::records::navigator::Navigator;
use crate::type_aliases::error_require_navigator::Error;
use luaur_config::records::config::Config;

impl Navigator {
    pub fn navigate_to_alias(
        &mut self,
        alias: &str,
        config: &Config,
        mut cycle_tracker: AliasCycleTracker,
    ) -> Error {
        debug_assert!(config.aliases.contains(&alias.to_string()));

        let value = config
            .aliases
            .find(&alias.to_string())
            .expect("alias must exist")
            .value
            .clone();
        let path_type = get_path_type(&value);

        if path_type == PathType::RelativeToCurrent || path_type == PathType::RelativeToParent {
            if let Some(error) = self.navigate_through_path(&value) {
                return Some(error);
            }
        } else if path_type == PathType::Aliased {
            if let Some(error) = cycle_tracker.add(alias.to_string()) {
                return Some(error);
            }

            let next_alias = extract_alias(&value);

            let (error, was_overridden) = self.to_alias_override(&next_alias);
            if error.is_some() {
                return error;
            } else if was_overridden {
                if let Some(error) = self.navigate_through_path(&value) {
                    return Some(error);
                }

                return None;
            }

            if config.aliases.contains(&next_alias) {
                if let Some(error) = self.navigate_to_alias(&next_alias, config, cycle_tracker) {
                    return Some(error);
                }
            } else {
                let mut parent_config = Config::default();
                if let Some(error) =
                    self.navigate_to_and_populate_config(&next_alias, &mut parent_config)
                {
                    return Some(error);
                }

                if parent_config.aliases.contains(&next_alias) {
                    let cycle_tracker = AliasCycleTracker {
                        seen: luaur_common::records::dense_hash_set::DenseHashSet::new(
                            alloc::string::String::new(),
                        ),
                        ordered: alloc::vec::Vec::new(),
                    };
                    if let Some(error) =
                        self.navigate_to_alias(&next_alias, &parent_config, cycle_tracker)
                    {
                        return Some(error);
                    }
                } else if let Some(error) = self.to_alias_fallback(&next_alias) {
                    return Some(error);
                }
            }

            if let Some(error) = self.navigate_through_path(&value) {
                return Some(error);
            }
        } else if let Some(error) = self.jump_to_alias(&value) {
            return Some(error);
        }

        None
    }
}
