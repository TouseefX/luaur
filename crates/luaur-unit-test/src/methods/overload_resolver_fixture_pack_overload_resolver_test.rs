use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;

impl OverloadResolverFixture {
    pub fn pack_vector_type_id(&self, tys: Vec<TypeId>) -> TypePackId {
        self.pack_initializer_list_type_id(&tys)
    }
}
