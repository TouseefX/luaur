use crate::enums::polarity::Polarity;
use crate::functions::fresh_index::fresh_index;
use crate::records::free_type::FreeType;
use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;

impl FreeType {
    pub fn free_type_scope_type_id_type_id_polarity(
        _scope: *mut Scope,
        _lower_bound: TypeId,
        _upper_bound: TypeId,
        _polarity: Polarity,
    ) -> Self {
        let index = fresh_index();
        let level = TypeLevel::default();
        let mut ft = FreeType {
            index,
            level,
            scope: _scope,
            forwarded_type_alias: false,
            lower_bound: _lower_bound,
            upper_bound: _upper_bound,
            polarity: _polarity,
        };
        ft.free_type_type_level_type_id_type_id(level, _lower_bound, _upper_bound);
        ft
    }
}
