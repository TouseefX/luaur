use luaur_config::records::lint_warning::LintWarning;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct LintResult {
    pub errors: alloc::vec::Vec<LintWarning>,
    pub warnings: alloc::vec::Vec<LintWarning>,
}
