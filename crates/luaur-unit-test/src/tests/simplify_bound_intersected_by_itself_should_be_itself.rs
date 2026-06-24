//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:606:simplify_bound_intersected_by_itself_should_be_itself`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_bound_intersected_by_itself_should_be_itself

#[cfg(test)]
#[test]
fn simplify_bound_intersected_by_itself_should_be_itself() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::blocked_type::BlockedType;

    let mut fixture = SimplifyFixture::default();
    let blocked = fixture.arena.add_type(BlockedType::default());

    let expected = to_string_type_id_to_string_options(blocked, &mut fixture.opts);
    let actual = fixture.intersect_str(blocked, blocked);
    assert_eq!(expected, actual);
}
