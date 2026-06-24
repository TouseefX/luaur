use crate::enums::report_format::ReportFormat;
use crate::functions::report::report;
use luaur_analysis::functions::to_string_error_alt_k::to_string_type_error_type_error_to_string_options;
use luaur_analysis::records::file_resolver::FileResolver;
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::syntax_error::SyntaxError;
use luaur_analysis::records::type_error::TypeError;
use luaur_analysis::records::type_error_to_string_options::TypeErrorToStringOptions;
use luaur_analysis::type_aliases::type_error_data::TypeErrorDataMember;

/// C++ `static void reportError(const Frontend& frontend, ReportFormat format, const TypeError& error)`
/// (`CLI/src/Analyze.cpp:70-84`).
pub fn report_error(frontend: &Frontend, format: ReportFormat, error: &TypeError) {
    // std::string humanReadableName = frontend.fileResolver->getHumanReadableModuleName(error.moduleName);
    let human_readable_name = unsafe {
        FileResolver::get_human_readable_module_name(frontend.file_resolver, &error.module_name)
    };

    // if (const SyntaxError* syntaxError = get_if<SyntaxError>(&error.data))
    if let Some(syntax_error) = SyntaxError::get_if(&error.data) {
        report(
            format,
            &human_readable_name,
            &error.location,
            "SyntaxError",
            syntax_error.message(),
        );
    } else {
        let message = to_string_type_error_type_error_to_string_options(
            error,
            TypeErrorToStringOptions {
                file_resolver: frontend.file_resolver,
            },
        );
        report(
            format,
            &human_readable_name,
            &error.location,
            "TypeError",
            &message,
        );
    }
}
