//! `DemoConfigResolver::getConfig` (`CLI/src/Web.cpp:56-59`).
//!
//! ```cpp
//! virtual const Luau::Config& getConfig(const Luau::ModuleName& name, const Luau::TypeCheckLimits& limits) const override
//! {
//!     return defaultConfig;
//! }
//! ```

use crate::records::demo_config_resolver::DemoConfigResolver;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_config::records::config::Config;

impl DemoConfigResolver {
    pub fn get_config(&self, _name: &ModuleName, _limits: &TypeCheckLimits) -> &Config {
        &self.default_config
    }
}
