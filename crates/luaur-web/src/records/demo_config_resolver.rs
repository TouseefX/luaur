//! Port of `DemoConfigResolver : Luau::ConfigResolver` (`CLI/src/Web.cpp:49-62`).
//!
//! The luau.org/demo config resolver, defaulting to `Mode::Strict`. Like the
//! analysis `ConfigResolver` (a struct with a fn-pointer vtable slot for the
//! single pure virtual `getConfig`), this concrete subclass is `#[repr(C)]` with
//! `base: ConfigResolver` first so a `*const ConfigResolver` (the vtable
//! receiver) can be cast back to `*const DemoConfigResolver` to reach
//! `default_config`.
//!
//! C++ member: `Luau::Config defaultConfig;`.

use luaur_analysis::records::config_resolver::ConfigResolver;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_config::records::config::Config;
use luaur_config::type_aliases::module_name::ModuleName;

#[repr(C)]
#[derive(Debug)]
pub struct DemoConfigResolver {
    pub base: ConfigResolver,
    pub default_config: Config,
}

/// `const Config& getConfig(const ModuleName&, const TypeCheckLimits&) const` thunk.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `DemoConfigResolver`.
pub(crate) unsafe fn demo_config_resolver_get_config_thunk(
    this: *const ConfigResolver,
    name: *const ModuleName,
    limits: *const TypeCheckLimits,
) -> *const Config {
    let this = this as *const DemoConfigResolver;
    (*this).get_config(&*name, &*limits) as *const Config
}
