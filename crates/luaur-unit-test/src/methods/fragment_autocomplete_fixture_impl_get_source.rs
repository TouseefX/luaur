use crate::records::fragment_autocomplete_fixture_impl::FragmentAutocompleteFixtureImpl;
use luaur_analysis::records::source_module::SourceModule;

impl FragmentAutocompleteFixtureImpl {
    pub fn get_source(&mut self) -> &mut SourceModule {
        self.base.base.source_module = Some(alloc::boxed::Box::new(SourceModule::source_module()));
        self.base
            .base
            .source_module
            .as_deref_mut()
            .expect("fragment source module was just initialized")
    }
}
