use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::records::check_result::CheckResult;
use luaur_analysis::records::frontend_options::FrontendOptions;
use luaur_ast::enums::mode::Mode;

const MAIN_MODULE_NAME: &str = "MainModule";

impl Fixture {
    pub fn check_mode_string_optional_frontend_options(
        &mut self,
        mode: Mode,
        source: &String,
        options: Option<FrontendOptions>,
    ) -> CheckResult {
        self.get_frontend();

        let module_name = String::from(MAIN_MODULE_NAME);
        self.config_resolver.default_config.mode = mode;
        self.file_resolver
            .source
            .insert(module_name.clone(), source.clone());

        let frontend = self.get_frontend();
        frontend.mark_dirty(&module_name, None);
        frontend.clear_stats();
        frontend.check_module_name_optional_frontend_options(&module_name, options)
    }
}
