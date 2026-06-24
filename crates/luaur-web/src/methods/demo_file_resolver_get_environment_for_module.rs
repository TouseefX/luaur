//! `DemoFileResolver::getEnvironmentForModule` (`CLI/src/Web.cpp:40-43`).
//!
//! ```cpp
//! std::optional<std::string> getEnvironmentForModule(const Luau::ModuleName& name) const override
//! {
//!     return std::nullopt;
//! }
//! ```

use crate::records::demo_file_resolver::DemoFileResolver;
use alloc::string::String;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;

impl DemoFileResolver {
    pub fn get_environment_for_module(&self, _name: &ModuleName) -> Option<String> {
        None
    }
}
