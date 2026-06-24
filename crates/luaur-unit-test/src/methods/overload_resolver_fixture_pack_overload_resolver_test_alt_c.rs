use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::records::type_pack_var::TypePackVar;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;
use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

impl OverloadResolverFixture {
    pub fn pack_initializer_list_type_id_type_pack_variant(
        &self,
        tys: &[TypeId],
        tail: TypePackVariant,
    ) -> TypePackId {
        unsafe {
            let tail_pack = (*self.arena).add_type_pack_type_pack_var(TypePackVar::new(tail));
            (*self.arena)
                .add_type_pack_vector_type_id_optional_type_pack_id(tys.to_vec(), Some(tail_pack))
        }
    }
}
