use crate::records::frontend::Frontend;
use crate::type_aliases::module_name_file_resolver::ModuleName;

impl Frontend {
    pub fn parse_module_name(&mut self, name: &ModuleName) {
        luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE!(
            "Frontend::parse",
            "Frontend"
        );
        luaur_common::macros::luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT!(
            "name",
            name.as_str()
        );

        if self.get_check_result(name, false, false).is_some() {
            return;
        }

        let mut build_queue: alloc::vec::Vec<ModuleName> = alloc::vec::Vec::new();
        self.parse_graph(
            &mut build_queue,
            name,
            &crate::records::type_check_limits::TypeCheckLimits::default(),
            false,
        );
    }
}
