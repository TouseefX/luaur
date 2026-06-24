use crate::records::type_checker::TypeChecker;
use crate::type_aliases::name_type_infer::Name;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn get_index_type_from_type(
        &mut self,
        _scope: ScopePtr,
        _type: TypeId,
        _name: &Name,
        _location: &Location,
        _add_errors: bool,
    ) -> Option<TypeId> {
        let error_count = unsafe { (*self.current_module.as_ref().unwrap()).errors.len() };
        let result =
            self.get_index_type_from_type_impl(_scope, _type, _name, _location, _add_errors);
        if !_add_errors {
            luaur_common::macros::luau_assert::LUAU_ASSERT!(
                error_count == unsafe { (*self.current_module.as_ref().unwrap()).errors.len() }
            );
        }
        result
    }
}
