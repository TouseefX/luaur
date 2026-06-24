use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type::FreeType;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;

impl TypeArena {
    pub fn fresh_type_not_null_builtin_types_scope(
        &mut self,
        builtins: &BuiltinTypes,
        scope: *mut Scope,
    ) -> TypeId {
        self.add_type(FreeType {
            scope,
            lower_bound: builtins.neverType,
            upper_bound: builtins.unknownType,
            ..FreeType::default()
        })
    }
}
