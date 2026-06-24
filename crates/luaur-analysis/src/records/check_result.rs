use crate::records::lint_result::LintResult;
use crate::records::type_error::TypeError;

#[derive(Debug, Clone, Default)]
pub struct CheckResult {
    pub errors: alloc::vec::Vec<TypeError>,
    pub lint_result: LintResult,
    pub timeout_hits: alloc::vec::Vec<crate::type_aliases::module_name_file_resolver::ModuleName>,
}
