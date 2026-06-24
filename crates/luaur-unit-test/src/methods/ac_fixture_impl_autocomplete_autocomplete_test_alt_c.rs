use crate::records::ac_fixture_impl::AcFixtureImpl;
use luaur_analysis::functions::autocomplete_autocomplete::autocomplete;
use luaur_analysis::records::autocomplete_result::AutocompleteResult;
use luaur_analysis::records::frontend_options::FrontendOptions;
use luaur_analysis::type_aliases::module_name_file_resolver::ModuleName;
use luaur_analysis::type_aliases::string_completion_callback::StringCompletionCallback;
use luaur_ast::enums::mode::Mode;
use luaur_ast::records::position::Position;

impl AcFixtureImpl {
    pub fn autocomplete_module_name_position_string_completion_callback(
        &mut self,
        name: &ModuleName,
        pos: Position,
        callback: StringCompletionCallback,
    ) -> AutocompleteResult {
        let mut opts = FrontendOptions::default();
        opts.for_autocomplete = true;
        opts.retain_full_type_graphs = true;

        self.base.config_resolver.default_config.mode = Mode::NoCheck;
        self.base.file_resolver.enable_require_suggester();

        {
            let frontend = self.get_frontend();
            frontend.check_module_name_optional_frontend_options(name, Some(opts));
        }

        autocomplete(self.get_frontend(), name, pos, callback)
    }
}
