use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type::FreeType;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;

impl TypeArena {
    pub fn fresh_type_not_null_builtin_types_type_level(
        &mut self,
        builtins: &BuiltinTypes,
        level: TypeLevel,
    ) -> TypeId {
        self.add_type(FreeType {
            level,
            lower_bound: builtins.neverType,
            upper_bound: builtins.unknownType,
            ..FreeType::default()
        })
    }
}
