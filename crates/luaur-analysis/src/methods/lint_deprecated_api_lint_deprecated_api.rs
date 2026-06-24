use crate::records::lint_context::LintContext;
use crate::records::lint_deprecated_api::LintDeprecatedApi;

impl LintDeprecatedApi {
    pub fn lint_deprecated_api_lint_deprecated_api(&mut self, context: *mut LintContext) {
        self.context = context;
    }
}
