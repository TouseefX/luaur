impl crate::records::lint_context::LintContext {
    pub fn warning_enabled(&mut self, code: luaur_config::enums::code::Code) -> bool {
        let code_val = code as u64;
        (self.options.warning_mask & (1u64 << code_val)) != 0
    }
}
