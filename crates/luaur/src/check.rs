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

/// The fixed package name under which host definitions are registered. Mirrors
/// `Fixture::loadDefinition`'s `"@test"`; `@`-prefixed names are the convention
/// for synthetic (non-file) modules.
const HOST_DEFINITIONS_PACKAGE: &str = "@host";

/// The fallible body, run under `catch_unwind` (like `check_script`) so a panic
/// in the type checker surfaces as a diagnostic rather than unwinding into the
/// caller. Returns the collected `"<line>: <message>"` diagnostics, or an empty
/// `Vec` when the source type-checks clean.
///
/// `definitions`, when present and non-empty, is Luau definition-file syntax
/// (`declare function …`, `declare class …`, `declare x: T`) describing the host
/// surface; it is registered into the global scope *after* the builtins (so it
/// may reference them) and *before* the script is checked, exactly as a `Fixture`
/// registers a definition file.
fn run_check(source: &str, definitions: Option<&str>) -> Vec<String> {
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

    // Register host type definitions, if any, into the same global scope the
    // builtins live in. This is the unique Luau capability: a script then
    // type-checks against the host-provided surface (the Rust functions /
    // userdata exposed to the runtime). Pattern mirrors `Fixture::loadDefinition`:
    // unfreeze -> loadDefinitionFile(globals, globalScope, …) -> freeze.
    if let Some(defs) = definitions {
        if !defs.is_empty() {
            unsafe {
                unfreeze((*frontend_ptr).globals.global_types_mut());
                let target_scope = (*frontend_ptr).globals.global_scope();
                let result = (*frontend_ptr).load_definition_file(
                    &mut (*frontend_ptr).globals,
                    target_scope,
                    defs,
                    String::from(HOST_DEFINITIONS_PACKAGE),
                    /* captureComments */ false,
                    /* typeCheckForAutocomplete */ false,
                );
                freeze((*frontend_ptr).globals.global_types_mut());

                // Malformed host definitions are a usage error, surfaced with a
                // "host definitions:" prefix so they are distinguishable from
                // script diagnostics. A failed load did not persist anything, so
                // checking the script against a half-built surface is pointless —
                // return immediately.
                if !result.success {
                    for err in &result.parse_result.errors {
                        let mut line = (err.get_location().begin.line + 1).to_string();
                        line.push_str(": host definitions: ");
                        line.push_str(err.get_message());
                        diagnostics.push(line);
                    }
                    if let Some(module) = &result.module {
                        for err in &module.errors {
                            let mut line = (err.location.begin.line + 1).to_string();
                            line.push_str(": host definitions: ");
                            line.push_str(&to_string_type_error(err));
                            diagnostics.push(line);
                        }
                    }
                    if diagnostics.is_empty() {
                        diagnostics.push("host definitions: failed to load".to_string());
                    }
                    return diagnostics;
                }
            }
        }
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
    check_inner(source, None)
}

/// Type-check Luau `source` with extra host type `definitions` in scope.
///
/// `definitions` is Luau **definition-file syntax** describing the host-provided
/// globals — the Rust functions, values, and userdata you expose to the runtime
/// (e.g. via [`luaur_rt`](crate)'s `create_function` / `UserData`):
///
/// ```text
/// declare function add(a: number, b: number): number
/// declare config: { name: string, retries: number }
/// declare class Vec2
///     x: number
///     y: number
///     function magnitude(self): number
/// end
/// ```
///
/// The Luau VM is dynamically typed, so the runtime does not need these — but the
/// *static* checker has no knowledge of the host surface unless you tell it. This
/// is a capability `mlua` cannot offer (Lua has no static types): a script you are
/// about to run can be type-checked against exactly the API the host exposes.
///
/// Returns `Ok(())` when the source type-checks clean against the builtins plus
/// the host definitions, or `Err` of the diagnostics (`"line: message"`, 1-based).
/// Errors in the definitions themselves are reported with a `"host definitions:"`
/// prefix.
///
/// ```
/// // The script references a host function the checker would otherwise reject:
/// luaur::check("local n: number = add(1, 2)").unwrap_err();
/// luaur::check_with_definitions(
///     "local n: number = add(1, 2)",
///     "declare function add(a: number, b: number): number",
/// )
/// .unwrap();
/// ```
pub fn check_with_definitions(source: &str, definitions: &str) -> Result<(), Vec<String>> {
    check_inner(source, Some(definitions))
}

/// Shared body of [`check`] / [`check_with_definitions`]: run the checker under
/// `catch_unwind` (so a panic in the type checker becomes a diagnostic rather
/// than unwinding into the caller) and fold the diagnostics into a `Result`.
fn check_inner(source: &str, definitions: Option<&str>) -> Result<(), Vec<String>> {
    // try { ... } catch (const std::exception& e) { ... } — a panic inside the
    // checker becomes a single diagnostic.
    let owned = source.to_string();
    let owned_defs = definitions.map(|d| d.to_string());
    let diagnostics =
        match std::panic::catch_unwind(move || run_check(&owned, owned_defs.as_deref())) {
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
    use super::{check, check_with_definitions};

    #[test]
    fn check_accepts_well_typed_assignment() {
        check("local x: number = 1").expect("well-typed source should check clean");
    }

    #[test]
    fn definitions_introduce_a_host_function() {
        // Without the declaration the global `add` is unknown; under --!strict
        // that is an error.
        let bare = check("--!strict\nlocal n: number = add(1, 2)\nreturn n");
        assert!(
            bare.is_err(),
            "undeclared host function should not type-check: {bare:?}"
        );

        // Declaring it makes the same script check clean — and with the right type.
        check_with_definitions(
            "--!strict\nlocal n: number = add(1, 2)\nreturn n",
            "declare function add(a: number, b: number): number",
        )
        .expect("declared host function should type-check");
    }

    #[test]
    fn definitions_are_type_checked_against() {
        // `add` returns number, so assigning it to a string must fail even though
        // the call itself is well-formed.
        let errors = check_with_definitions(
            "--!strict\nlocal s: string = add(1, 2)\nreturn s",
            "declare function add(a: number, b: number): number",
        )
        .expect_err("number result assigned to string should fail");
        let joined = errors.join("\n");
        assert!(
            joined.contains("number") && joined.contains("string"),
            "diagnostic should mention the number/string mismatch: {joined}"
        );
    }

    #[test]
    fn definitions_can_declare_a_host_value() {
        check_with_definitions(
            "--!strict\nlocal name: string = config.name\nreturn name",
            "declare config: { name: string, retries: number }",
        )
        .expect("declared host value field access should type-check");
    }

    #[test]
    fn malformed_definitions_are_reported_with_prefix() {
        let errors = check_with_definitions(
            "return 1",
            "declare function add(a: number, b: number: number", // missing ')'
        )
        .expect_err("malformed host definitions should fail");
        let joined = errors.join("\n");
        assert!(
            joined.contains("host definitions:"),
            "definition errors should carry the host-definitions prefix: {joined}"
        );
    }

    #[test]
    fn empty_definitions_behave_like_plain_check() {
        check_with_definitions("local x: number = 1", "")
            .expect("empty definitions should be a no-op");
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
