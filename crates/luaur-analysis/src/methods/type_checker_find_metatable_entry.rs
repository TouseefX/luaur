use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::records::type_checker::TypeChecker;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn find_metatable_entry(
        &mut self,
        ty: TypeId,
        entry: alloc::string::String,
        location: &Location,
        add_errors: bool,
    ) -> Option<TypeId> {
        let mut errors: ErrorVec = ErrorVec::new();
        let result = find_metatable_entry(self.builtin_types, &mut errors, ty, &entry, *location);
        if add_errors {
            self.report_errors(&errors);
        }
        result
    }
}
