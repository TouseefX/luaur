use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::records::check_result::CheckResult;
use luaur_analysis::records::frontend_options::FrontendOptions;
use luaur_ast::enums::mode::Mode;

impl Fixture {
    pub fn check_string_optional_frontend_options(
        &mut self,
        source: &String,
        options: Option<FrontendOptions>,
    ) -> CheckResult {
        self.check_mode_string_optional_frontend_options(Mode::Strict, source, options)
    }
}
