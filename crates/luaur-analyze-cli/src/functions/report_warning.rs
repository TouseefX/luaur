use crate::enums::report_format::ReportFormat;
use crate::functions::report::report;
use luaur_config::records::lint_warning::LintWarning;

/// C++ `static void reportWarning(ReportFormat format, const char* name, const LintWarning& warning)`
/// (`CLI/src/Analyze.cpp:86-89`).
pub fn report_warning(format: ReportFormat, name: &str, warning: &LintWarning) {
    report(
        format,
        name,
        &warning.location,
        LintWarning::get_name(warning.code),
        &warning.text,
    );
}
