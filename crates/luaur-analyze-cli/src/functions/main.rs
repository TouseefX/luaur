use crate::enums::report_format::ReportFormat;
use crate::functions::assertion_handler::assertion_handler;
use crate::functions::display_help::display_help;
use crate::functions::report::report;
use crate::functions::report_module_result::report_module_result;
use crate::records::cli_config_resolver::CliConfigResolver;
use crate::records::cli_file_resolver::CliFileResolver;
use crate::records::task_scheduler::{Task, TaskScheduler};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;
use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::functions::freeze::freeze;
use luaur_analysis::functions::register_builtin_globals::register_builtin_globals;
use luaur_analysis::functions::to_string_error_alt_k::to_string_type_error_type_error_to_string_options;
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::frontend_options::FrontendOptions;
use luaur_analysis::records::internal_compiler_error::InternalCompilerError;
use luaur_analysis::records::internal_error::InternalError;
use luaur_analysis::records::type_error::TypeError;
use luaur_analysis::records::type_error_to_string_options::TypeErrorToStringOptions;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_ast::enums::mode::Mode;
use luaur_ast::records::location::Location;
use luaur_cli_lib::functions::get_source_files::get_source_files;
use luaur_cli_lib::functions::set_luau_flags_default::set_luau_flags_default;
use luaur_cli_lib::functions::set_luau_flags_flags_alt_b::set_luau_flags_c_char;

/// A `Box<dyn Fn()>` carried across the thread boundary. The C++ `TaskScheduler`
/// stores `std::function<void()>` with no `Send` concept; the queued tasks capture
/// shared frontend state and run on worker threads exactly as here.
struct SendTask(Box<dyn Fn()>);
unsafe impl Send for SendTask {}

/// C++ `int main(int argc, char** argv)` (`CLI/src/Analyze.cpp:394-542`).
pub fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    // Build an owned argv of NUL-terminated C strings so the FileUtils/Flags ports
    // (which take `int argc, char** argv`) can be called faithfully.
    let owned_args: Vec<std::ffi::CString> = std::env::args()
        .map(|a| std::ffi::CString::new(a).unwrap_or_else(|_| std::ffi::CString::new("").unwrap()))
        .collect();
    let mut argv: Vec<*mut c_char> = owned_args
        .iter()
        .map(|c| c.as_ptr() as *mut c_char)
        .collect();
    argv.push(core::ptr::null_mut());
    let argc = owned_args.len() as i32;

    // Luau::assertHandler() = assertionHandler;
    *luaur_common::functions::assert_handler::assert_handler() = Some(assertion_handler);

    // setLuauFlagsDefault();
    set_luau_flags_default();

    // if (argc >= 2 && strcmp(argv[1], "--help") == 0) { displayHelp(argv[0]); return 0; }
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "--help" {
        display_help(&args[0]);
        return 0;
    }

    let mut format = ReportFormat::Default;
    let mut mode = Mode::Nonstrict;
    let mut annotate = false;
    let mut thread_count: i32 = 0;
    let mut base_path = String::new();
    let mut solver_mode = SolverMode::New;

    // for (int i = 1; i < argc; ++i)
    for arg in args.iter().skip(1) {
        if !arg.starts_with('-') {
            continue;
        }

        if arg == "--formatter=plain" {
            format = ReportFormat::Luacheck;
        } else if arg == "--formatter=gnu" {
            format = ReportFormat::Gnu;
        } else if arg == "--mode=strict" {
            mode = Mode::Strict;
        } else if arg == "--annotate" {
            annotate = true;
        } else if arg == "--timetrace" {
            luaur_common::FFlag::DebugLuauTimeTracing.set(true);
        } else if let Some(rest) = arg.strip_prefix("--fflags=") {
            let c = std::ffi::CString::new(rest)
                .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
            set_luau_flags_c_char(c.as_ptr());
        } else if let Some(rest) = arg.strip_prefix("-j") {
            thread_count = rest.parse::<i32>().unwrap_or(0);
        } else if let Some(rest) = arg.strip_prefix("--logbase=") {
            base_path = String::from(rest);
        } else if arg == "--solver=old" {
            solver_mode = SolverMode::Old;
        }
    }

    // The Rust build does not define LUAU_ENABLE_TIME_TRACE; mirror the C++ guard.
    if luaur_common::FFlag::DebugLuauTimeTracing.get() {
        eprintln!(
            "To run with --timetrace, Luau has to be built with LUAU_ENABLE_TIME_TRACE enabled"
        );
        return 1;
    }

    // FrontendOptions frontendOptions; retainFullTypeGraphs = annotate; runLintChecks = true;
    let mut frontend_options = FrontendOptions::default();
    frontend_options.retain_full_type_graphs = annotate;
    frontend_options.run_lint_checks = true;

    // CliFileResolver fileResolver; CliConfigResolver configResolver(mode);
    let mut file_resolver = CliFileResolver::new();
    let mut config_resolver = CliConfigResolver::cli_config_resolver(mode);

    // Frontend frontend(solverMode, &fileResolver, &configResolver, frontendOptions);
    let mut frontend =
        Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
            solver_mode,
            &mut file_resolver.base,
            &mut config_resolver.base,
            frontend_options,
        );
    // Re-establish the resolver pointers and the self-referential pointers now that
    // `frontend` lives at a stable address (mirrors the project's wiring convention).
    frontend.file_resolver = &mut file_resolver.base;
    frontend.config_resolver = &mut config_resolver.base;
    unsafe {
        frontend.wire_self_pointers();
    }

    // if (FFlag::DebugLuauLogSolverToJsonFile) { frontend.writeJsonLog = ...; }
    if luaur_common::FFlag::DebugLuauLogSolverToJsonFile.get() {
        let base_path = base_path.clone();
        frontend.write_json_log = Some(alloc::rc::Rc::new(
            move |module_name: &ModuleName, log: String| {
                let mut path = alloc::format!("{}.log.json", module_name);
                if let Some(pos) = module_name.rfind('/') {
                    path = String::from(&module_name[pos + 1..]);
                }
                if !base_path.is_empty() {
                    path = luaur_cli_lib::functions::join_paths_file_utils_alt_b::join_paths_string_view_string_view(&base_path, &path);
                }
                if std::fs::write(&path, alloc::format!("{}\n", log)).is_ok() {
                    println!("Wrote JSON log to {}", path);
                }
            },
        ));
    }

    // registerBuiltinGlobals(frontend, frontend.globals);
    // freeze(frontend.globals.globalTypes);
    unsafe {
        let frontend_ptr: *mut Frontend = &mut frontend;
        register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
        freeze((*frontend_ptr).globals.global_types_mut());
    }

    // std::vector<std::string> files = getSourceFiles(argc, argv);
    let files = get_source_files(argc, argv.as_mut_ptr());

    // for (const std::string& path : files) frontend.queueModuleCheck(path);
    frontend.queue_module_check_vector_module_name(&files);

    let mut checked_modules: Vec<ModuleName>;

    // if (threadCount <= 0) threadCount = std::min(getThreadCount(), 8u);
    if thread_count <= 0 {
        thread_count = core::cmp::min(TaskScheduler::get_thread_count(), 8) as i32;
    }

    // try { TaskScheduler scheduler(threadCount); checkedModules = frontend.checkQueuedModules(...); }
    let frontend_ptr: *mut Frontend = &mut frontend;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let scheduler = TaskScheduler::task_scheduler_task_scheduler(thread_count as u32);
        let scheduler_ptr: *const TaskScheduler = &scheduler;

        // The executor pushes each task onto the scheduler queue, matching:
        //   [&](std::vector<std::function<void()>> tasks) { for (auto& t : tasks) scheduler.push(std::move(t)); }
        let execute_tasks: Box<dyn Fn(Vec<Box<dyn Fn()>>)> = Box::new(move |tasks| {
            for task in tasks {
                let send_task = SendTask(task);
                let boxed: Task = Some(Box::new(move || {
                    // Move the whole `SendTask` (which is `Send`) into the closure so
                    // the auto-trait analysis sees `Send`, rather than capturing only
                    // the inner non-`Send` `Box<dyn Fn()>` (edition-2021 disjoint capture).
                    let send_task = send_task;
                    (send_task.0)();
                }));
                crate::methods::task_scheduler_push::task_scheduler_push(
                    unsafe { &*scheduler_ptr },
                    boxed,
                );
            }
        });

        let progress: Box<dyn Fn(usize, usize) -> bool> = Box::new(|_done, _total| true);

        let modules =
            unsafe { (*frontend_ptr).check_queued_modules(None, execute_tasks, progress) };

        // scheduler is dropped here (joins workers), matching the C++ block scope.
        drop(scheduler);
        modules
    }));

    match result {
        Ok(modules) => checked_modules = modules,
        Err(payload) => {
            // catch (const InternalCompilerError& ice)
            let ice: InternalCompilerError =
                if let Some(e) = payload.downcast_ref::<InternalCompilerError>() {
                    e.clone()
                } else if let Some(e) = payload
                    .downcast_ref::<luaur_analysis::records::time_limit_error::TimeLimitError>(
                ) {
                    e.base.clone()
                } else if let Some(e) = payload
                    .downcast_ref::<luaur_analysis::records::user_cancel_error::UserCancelError>(
                ) {
                    e.base.clone()
                } else {
                    std::panic::resume_unwind(payload);
                };

            let location = ice.location.unwrap_or_else(Location::default);
            let module_name = ice
                .module_name
                .clone()
                .unwrap_or_else(|| String::from("<unknown module>"));
            let human_readable_name = unsafe {
                luaur_analysis::records::file_resolver::FileResolver::get_human_readable_module_name(
                    frontend.file_resolver,
                    &module_name,
                )
            };

            let error = TypeError::type_error_location_module_name_type_error_data(
                location.clone(),
                module_name,
                InternalError::new(ice.message.clone()).into(),
            );

            let message = to_string_type_error_type_error_to_string_options(
                &error,
                TypeErrorToStringOptions {
                    file_resolver: frontend.file_resolver,
                },
            );
            report(
                format,
                &human_readable_name,
                &location,
                "InternalCompilerError",
                &message,
            );
            return 1;
        }
    }

    let mut failed = 0i32;

    // for (const ModuleName& name : checkedModules) failed += !reportModuleResult(...);
    let names = core::mem::take(&mut checked_modules);
    for name in &names {
        if !report_module_result(&mut frontend, name, format, annotate) {
            failed += 1;
        }
    }

    // if (!configResolver.configErrors.empty()) { ... }
    if !config_resolver.config_errors.is_empty() {
        failed += config_resolver.config_errors.len() as i32;

        for (path, error) in &config_resolver.config_errors {
            eprintln!("{}: {}", path, error);
        }
    }

    if format == ReportFormat::Luacheck {
        0
    } else if failed != 0 {
        1
    } else {
        0
    }
}
