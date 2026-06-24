use crate::enums::config_behavior::ConfigBehavior;
use crate::enums::config_status::ConfigStatus;
use crate::enums::navigate_result::NavigateResult;
use crate::records::navigator::Navigator;
use crate::type_aliases::error_require_navigator::Error;
use alloc::string::String;
use luaur_config::functions::extract_luau_config::extract_luau_config;
use luaur_config::functions::parse_config::parse_config;
use luaur_config::records::alias_options::AliasOptions;
use luaur_config::records::config::Config;
use luaur_config::records::config_options::ConfigOptions;
use luaur_config::records::interrupt_callbacks::InterruptCallbacks;

impl Navigator {
    pub fn navigate_to_and_populate_config(
        &mut self,
        desired_alias: &str,
        config: &mut Config,
    ) -> Error {
        let desired_alias = String::from(desired_alias);

        while !config.aliases.contains(&desired_alias) {
            *config = Config::default();

            let navigation_context = unsafe { &mut *self.navigation_context };
            let result = navigation_context.to_parent();
            if result == NavigateResult::Ambiguous {
                return Some(format!(
                    "could not navigate up the ancestry chain during search for alias \"{}\" (ambiguous)",
                    desired_alias
                ));
            }
            if result == NavigateResult::NotFound {
                break;
            }

            let status = navigation_context.get_config_status();
            if status == ConfigStatus::Absent {
                continue;
            } else if status == ConfigStatus::Ambiguous {
                return Some(format!(
                    "could not resolve alias \"{}\" (ambiguous configuration file)",
                    desired_alias
                ));
            } else {
                if navigation_context.get_config_behavior() == ConfigBehavior::GetAlias {
                    let alias_path = navigation_context.get_alias(&desired_alias);
                    if luaur_common::FFlag::LuauRequireResolveAliasNullCheck.get()
                        && alias_path.is_none()
                    {
                        return Some(format!("could not resolve alias \"{}\"", desired_alias));
                    }

                    config.set_alias_simple(desired_alias.clone(), alias_path.unwrap());
                    break;
                }

                let Some(config_contents) = navigation_context.get_config() else {
                    return Some(format!(
                        "could not get configuration file contents to resolve alias \"{}\"",
                        desired_alias
                    ));
                };

                let mut opts = ConfigOptions::default();
                let mut alias_opts = AliasOptions::default();
                alias_opts.overwrite_aliases = false;
                opts.alias_options = Some(alias_opts);

                if status == ConfigStatus::PresentJson {
                    if let Some(error) = parse_config(&config_contents, config, &opts) {
                        return Some(error);
                    }
                } else if status == ConfigStatus::PresentLuau {
                    let callbacks = InterruptCallbacks {
                        init_callback: navigation_context.luau_config_init(),
                        interrupt_callback: navigation_context.luau_config_interrupt(),
                    };

                    if let Some(error) = extract_luau_config(
                        &config_contents,
                        config,
                        opts.alias_options.take(),
                        callbacks,
                    ) {
                        return Some(error);
                    }
                }
            }
        }

        None
    }
}
