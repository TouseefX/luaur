use crate::records::warning_comparator::WarningComparator;
use luaur_config::records::lint_warning::LintWarning;

impl crate::records::warning_comparator::WarningComparator {
    #[inline]
    pub fn operator_call(&self) -> i32 {
        0
    }
}
