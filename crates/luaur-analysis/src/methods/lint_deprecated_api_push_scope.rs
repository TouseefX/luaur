use crate::records::function_type::FunctionType;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_common::LUAU_ASSERT;

impl LintDeprecatedApi {
    pub fn push_scope(&mut self, fty: *const FunctionType) {
        LUAU_ASSERT!(!fty.is_null());
        self.function_type_scope_stack.push(fty);
    }
}
