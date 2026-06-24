//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1053:normalize_intersect_with_not_unknown`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record NegationType (Analysis/include/Luau/Type.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item normalize_intersect_with_not_unknown

#[cfg(test)]
#[test]
fn normalize_intersect_with_not_unknown() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::negation_type::NegationType;

    let mut fixture = NormalizeFixture::default();
    let (unknown_type, number_type) = {
        let builtins = fixture.base.get_builtins();
        (builtins.unknownType, builtins.numberType)
    };

    let not_unknown = fixture.arena.add_type(NegationType::new(unknown_type));
    let ty = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![number_type, not_unknown],
    });
    let normalized = fixture.normalize(ty).expect("expected normalized type");

    assert_eq!(
        "never",
        to_string_type_id(fixture.type_from_normal(normalized.as_ref()))
    );
}
