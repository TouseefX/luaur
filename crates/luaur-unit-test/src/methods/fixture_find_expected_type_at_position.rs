use crate::records::fixture::Fixture;
use luaur_analysis::functions::find_expected_type_at_position::find_expected_type_at_position;
use luaur_analysis::records::module::Module;
use luaur_analysis::records::source_module::SourceModule;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_ast::records::position::Position;

impl Fixture {
    pub fn find_expected_type_at_position(&mut self, position: Position) -> Option<TypeId> {
        let module: *mut Module = self.get_main_module(false);
        let source_module: *mut SourceModule = self.get_main_source_module();
        unsafe { find_expected_type_at_position(&*module, &*source_module, position) }
    }
}
