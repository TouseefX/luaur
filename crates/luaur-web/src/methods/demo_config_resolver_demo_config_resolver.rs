//! `DemoConfigResolver::DemoConfigResolver()` (`CLI/src/Web.cpp:51-54`).
//!
//! ```cpp
//! DemoConfigResolver()
//! {
//!     defaultConfig.mode = Luau::Mode::Strict;
//! }
//! ```
//!
//! Wires the `ConfigResolver` vtable slot (`getConfig`) to the demo thunk.
//!
//! DELIBERATE DEVIATION from `Web.cpp`, which hard-sets `Mode::Strict`: the
//! playground defaults to `Nonstrict` so each script's own `--!strict` /
//! `--!nonstrict` mode comment governs (matching the `luaur-analyze` CLI). This
//! avoids type-checking unannotated example scripts under strict and reporting
//! findings that read, to a casual visitor, as the checker flagging clean code.

use crate::records::demo_config_resolver::{
    demo_config_resolver_get_config_thunk, DemoConfigResolver,
};
use luaur_analysis::records::config_resolver::ConfigResolver;
use luaur_ast::enums::mode::Mode;
use luaur_config::records::config::Config;

impl DemoConfigResolver {
    pub fn demo_config_resolver() -> Self {
        let mut default_config = Config::default();
        default_config.mode = Mode::Nonstrict;

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
