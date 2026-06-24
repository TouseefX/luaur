use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::records::intersection_type::IntersectionType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl OverloadResolverFixture {
    pub fn meet_initializer_list_type_id(&self, parts: &[TypeId]) -> TypeId {
        let intersection = IntersectionType {
            parts: parts.to_vec(),
        };
        unsafe { (*self.arena).add_type(intersection) }
    }
}
