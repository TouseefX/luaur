use crate::functions::fresh_index::fresh_index;
use crate::records::free_type::FreeType;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;

impl FreeType {
    pub fn free_type_scope_type_level_type_id_type_id(
        &mut self,
        _scope: *mut Scope,
        _level: TypeLevel,
        _lower_bound: TypeId,
        _upper_bound: TypeId,
    ) {
        self.index = fresh_index();
        self.level = _level;
        self.scope = _scope;
        self.lower_bound = _lower_bound;
        self.upper_bound = _upper_bound;
    }
}
