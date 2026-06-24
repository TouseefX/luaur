use crate::records::documentation_symbol_fixture::DocumentationSymbolFixture;
use alloc::string::ToString;
use luaur_analysis::functions::get_documentation_symbol_at_position::get_documentation_symbol_at_position;
use luaur_analysis::type_aliases::documentation_symbol::DocumentationSymbol;
use luaur_ast::records::position::Position;

impl DocumentationSymbolFixture {
    pub fn get_doc_symbol(
        &mut self,
        source: &str,
        position: Position,
    ) -> Option<DocumentationSymbol> {
        self.base.get_frontend();
        self.base
            .base
            .check_string_optional_frontend_options(&source.to_string(), None);

        let source_module = self.base.base.get_main_source_module();
        let module = self.base.base.get_main_module(false);

        unsafe { get_documentation_symbol_at_position(&*source_module, &*module, position) }
    }
}
