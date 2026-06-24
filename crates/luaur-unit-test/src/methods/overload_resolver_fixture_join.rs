use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::records::union_type::UnionType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl OverloadResolverFixture {
    pub fn join(&self, a: TypeId, b: TypeId) -> TypeId {
        unsafe {
            (*self.arena).add_type(UnionType {
                options: vec![a, b],
            })
        }
    }
}
