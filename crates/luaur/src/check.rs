//! Type-checking convenience helper, completing the compile/eval/check trio.
//!
//! Modelled exactly on `luaur-web`'s `check_script` (`CLI/src/Web.cpp:142-182`):
//! build a [`Frontend`] over an in-memory single-source file resolver, register
//! the Luau builtins, insert the source as the module `"main"`, type-check it on
//! the validated **old** solver, and surface each diagnostic as
//! `"<line>: <message>"` with a 1-based line number.
//!
//! `check_script` borrows `luaur-web`'s `DemoFileResolver`/`DemoConfigResolver`.
//! The umbrella crate must not depend on `luaur-web`, so a minimal single-source
//! in-memory resolver is replicated here: a `#[repr(C)]` struct whose first field
//! is the analysis `FileResolver` (so a `*mut FileResolver` vtable receiver casts
//! back to the concrete type) plus the four vtable thunks, and a matching
//! `ConfigResolver` returning a default [`Config`]. No require support is needed
//! for a string check, so `resolve_module` returns `None`.

use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::functions::freeze::freeze;
use luaur_analysis::functions::register_builtin_globals::register_builtin_globals;
use luaur_analysis::functions::to_string_error::to_string_type_error;
use luaur_analysis::functions::unfreeze::unfreeze;
use luaur_analysis::records::config_resolver::ConfigResolver;
use luaur_analysis::records::file_resolver::{FileResolver, FileResolverVtable};
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::frontend_options::FrontendOptions;
use luaur_analysis::records::module_info::ModuleInfo;
use luaur_analysis::records::source_code::SourceCode;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_config::records::config::Config;

/// The fixed module name under which the checked source is registered, matching
/// `check_script`'s `fileResolver.source["main"] = source`.
const MAIN_MODULE: &str = "main";

/// Minimal single-source in-memory [`FileResolver`] for a string check.
///
/// `#[repr(C)]` with `base` first so the vtable thunks can cast the
/// `*mut FileResolver` receiver back to `*mut CheckFileResolver` and reach
/// `source`. Holds exactly one module's source ("main").
#[repr(C)]
struct CheckFileResolver {
    base: FileResolver,
    source: String,
}

/// `readSource` thunk — returns the single source for the `"main"` module.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CheckFileResolver`.
unsafe fn check_file_resolver_read_source_thunk(
    this: *mut FileResolver,
    name: &ModuleName,
) -> Option<SourceCode> {
    let this = this as *const CheckFileResolver;
    if name != MAIN_MODULE {
        return None;
    }
    Some(SourceCode {
        source: (*this).source.clone(),
        r#type: SourceCode::Module,
    })
}

/// `resolveModule` thunk — no require support for a string check, so always
/// `None`.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CheckFileResolver`.
unsafe fn check_file_resolver_resolve_module_thunk(
    _this: *mut FileResolver,
    _context: *const ModuleInfo,
    _expr: *mut AstExpr,
    _limits: &TypeCheckLimits,
) -> Option<ModuleInfo> {
    None
}

/// `getHumanReadableModuleName` thunk — returns the name verbatim.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CheckFileResolver`.
unsafe fn check_file_resolver_get_human_readable_module_name_thunk(
    _this: *const FileResolver,
    name: &ModuleName,
) -> String {
    name.clone()
}

/// `getEnvironmentForModule` thunk — no per-module environment, so `None`.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CheckFileResolver`.
unsafe fn check_file_resolver_get_environment_for_module_thunk(
    _this: *const FileResolver,
    _name: &ModuleName,
) -> Option<String> {
    None
}

impl CheckFileResolver {
    fn new(source: &str) -> Self {
        let vtable = FileResolverVtable {
            read_source: check_file_resolver_read_source_thunk,
            resolve_module: check_file_resolver_resolve_module_thunk,
            get_human_readable_module_name: check_file_resolver_get_human_readable_module_name_thunk,
            get_environment_for_module: check_file_resolver_get_environment_for_module_thunk,
        };
        CheckFileResolver {
            base: FileResolver {
                vtable,
                require_suggester: None,
            },
            source: source.to_string(),
        }
    }
}

/// Minimal [`ConfigResolver`] returning a default [`Config`].
///
/// `#[repr(C)]` with `base` first so the `getConfig` thunk can cast the
/// `*const ConfigResolver` receiver back to `*const CheckConfigResolver`.
#[repr(C)]
struct CheckConfigResolver {
    base: ConfigResolver,
    default_config: Config,
}

/// `getConfig` thunk — returns the single default config.
///
/// # Safety
/// `this` must point at the `base` subobject of a live `CheckConfigResolver`.
unsafe fn check_config_resolver_get_config_thunk(
    this: *const ConfigResolver,
    _name: *const ModuleName,
    _limits: *const TypeCheckLimits,
) -> *const Config {
    let this = this as *const CheckConfigResolver;
    &(*this).default_config as *const Config
}

impl CheckConfigResolver {
    fn new() -> Self {
        CheckConfigResolver {
            base: ConfigResolver {
                get_config: Some(check_config_resolver_get_config_thunk),
            },
            default_config: Config::default(),
        }
    }
}

/// The fallible body, run under `catch_unwind` (like `check_script`) so a panic
/// in the type checker surfaces as a diagnostic rather than unwinding into the
/// caller. Returns the collected `"<line>: <message>"` diagnostics, or an empty
/// `Vec` when the source type-checks clean.
fn run_check(source: &str) -> Vec<String> {
    let mut diagnostics = Vec::new();

    let mut file_resolver = CheckFileResolver::new(source);
    let mut config_resolver = CheckConfigResolver::new();
    let options = FrontendOptions::default();

    let mut frontend = Frontend::frontend_file_resolver_config_resolver_frontend_options(
        &mut file_resolver.base,
        &mut config_resolver.base,
        &options,
    );
    unsafe {
        frontend.wire_self_pointers();
    }

    // Use the validated OLD solver path.
    frontend.set_luau_solver_mode(SolverMode::Old);

    // Add Luau builtins:
    //   Luau::unfreeze(frontend.globals.globalTypes);
    //   Luau::registerBuiltinGlobals(frontend, frontend.globals);
    //   Luau::freeze(frontend.globals.globalTypes);
    let frontend_ptr = &mut frontend as *mut Frontend;
    unsafe {
        unfreeze((*frontend_ptr).globals.global_types_mut());
        register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
        freeze((*frontend_ptr).globals.global_types_mut());
    }

    // Luau::CheckResult checkResult = frontend.check("main");
    let check_result =
        frontend.check_module_name_optional_frontend_options(&MAIN_MODULE.to_string(), None);

    for err in &check_result.errors {
        // std::to_string(err.location.begin.line + 1) + ": " + Luau::toString(err)
        let mut line = (err.location.begin.line + 1).to_string();
        line.push_str(": ");
        line.push_str(&to_string_type_error(err));
        diagnostics.push(line);
    }

    diagnostics
}

/// Type-check Luau `source`. Returns `Ok(())` if it type-checks clean, or `Err`
/// of the diagnostics (`"line: message"`, 1-based) on type errors.
///
/// ```
/// luaur::check("local x: number = 1").unwrap();
/// assert!(luaur::check("local x: number = \"oops\"").is_err());
/// ```
pub fn check(source: &str) -> Result<(), Vec<String>> {
    // try { ... } catch (const std::exception& e) { ... } — a panic inside the
    // checker becomes a single diagnostic.
    let owned = source.to_string();
    let diagnostics = match std::panic::catch_unwind(move || run_check(&owned)) {
        Ok(diagnostics) => diagnostics,
        Err(payload) => vec![panic_message(&payload)],
    };

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

/// Extract a `std::exception::what()`-equivalent message from a caught panic
/// payload.
fn panic_message(payload: &(dyn core::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown error".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn check_accepts_well_typed_assignment() {
        check("local x: number = 1").expect("well-typed source should check clean");
    }

    #[test]
    fn check_rejects_type_mismatch() {
        let errors = check("local x: number = \"oops\"").expect_err("type mismatch should fail");
        let joined = errors.join("\n");
        assert!(
            joined.contains("number") && joined.contains("string"),
            "diagnostic should mention number/string mismatch: {joined}"
        );
    }

    #[test]
    fn check_handles_missing_field_without_panicking() {
        // Behaves either way (Ok or Err) — the contract is just "no panic".
        let _ = check("--!strict\nlocal t = {}\nreturn t.missing");
    }
}
