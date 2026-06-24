use crate::methods::cli_config_resolver_get_config::cli_config_resolver_get_config_thunk;
use crate::records::cli_config_resolver::CliConfigResolver;
use luaur_analysis::records::config_resolver::ConfigResolver;
use luaur_ast::enums::mode::Mode;
use luaur_config::records::config::Config;
use std::collections::HashMap;

impl CliConfigResolver {
    /// C++ `CliConfigResolver(Luau::Mode mode) { defaultConfig.mode = mode; }`
    /// (`CLI/src/Analyze.cpp:238-241`).
    pub fn cli_config_resolver(mode: Mode) -> Self {
        let mut default_config = Config::default();
        default_config.mode = mode;

        CliConfigResolver {
            base: ConfigResolver {
                get_config: Some(cli_config_resolver_get_config_thunk),
            },
            default_config,
            config_cache: HashMap::new(),
            config_errors: alloc::vec::Vec::new(),
        }
    }
}
