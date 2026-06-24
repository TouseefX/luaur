use crate::records::frontend::Frontend;
use crate::records::lint_result::LintResult;
use luaur_config::records::config::Config;
use luaur_config::records::lint_warning::LintWarning;

impl Frontend {
    pub fn classify_lints(&self, warnings: &Vec<LintWarning>, config: &Config) -> LintResult {
        let mut result = LintResult::default();

        for w in warnings.iter() {
            let should_error = config.lint_errors || config.fatal_lint.is_enabled(w.code);
            if should_error {
                result.errors.push(w.clone());
            } else {
                result.warnings.push(w.clone());
            }
        }

        result
    }
}
