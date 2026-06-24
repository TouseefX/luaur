use crate::records::subtype_fixture::SubtypeFixture;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl SubtypeFixture {
    pub fn generic_pack_fn(
        &mut self,
        generic_packs: Vec<TypePackId>,
        arg_types: TypePackId,
        ret_types: TypePackId,
    ) -> TypeId {
        self.arena.add_type(FunctionType::new_with_generics(
            Vec::new(),
            generic_packs,
            arg_types,
            ret_types,
            None,
            false,
        ))
    }
}
