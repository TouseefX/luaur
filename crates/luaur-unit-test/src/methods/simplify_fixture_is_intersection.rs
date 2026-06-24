use crate::records::simplify_fixture::SimplifyFixture;
use luaur_analysis::functions::follow_type::follow_type_id;
use luaur_analysis::functions::get_type_alt_j::get_type_id;
use luaur_analysis::records::intersection_type::IntersectionType;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SimplifyFixture {
    pub fn is_intersection(&mut self, a: TypeId) -> bool {
        let followed = unsafe { follow_type_id(a) };
        let intersection = unsafe { get_type_id::<IntersectionType>(followed) };
        !intersection.is_null()
    }
}
