//! `DemoConfigResolver::DemoConfigResolver()` (`CLI/src/Web.cpp:51-54`).
//!
//! ```cpp
//! DemoConfigResolver()
//! {
//!     defaultConfig.mode = Luau::Mode::Strict;
//! }
//! ```
//!
//! Wires the `ConfigResolver` vtable slot (`getConfig`) to the demo thunk and
//! sets the default config's mode to `Strict`.

use crate::records::demo_config_resolver::{
    demo_config_resolver_get_config_thunk, DemoConfigResolver,
};
use luaur_analysis::records::config_resolver::ConfigResolver;
use luaur_ast::enums::mode::Mode;
use luaur_config::records::config::Config;

impl DemoConfigResolver {
    pub fn demo_config_resolver() -> Self {
        let mut default_config = Config::default();
        default_config.mode = Mode::Strict;

        DemoConfigResolver {
            base: ConfigResolver {
                get_config: Some(demo_config_resolver_get_config_thunk),
            },
            default_config,
        }
    }
}

impl Default for DemoConfigResolver {
    fn default() -> Self {
        Self::demo_config_resolver()
    }
}
