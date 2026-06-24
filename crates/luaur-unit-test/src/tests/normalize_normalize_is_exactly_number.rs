//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:986:normalize_normalize_is_exactly_number`
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
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method NormalizedType::isExactlyNumber (Analysis/src/Normalize.cpp)
//!   - calls -> function isNumber (Analysis/src/Type.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item normalize_normalize_is_exactly_number

#[cfg(test)]
#[test]
fn normalize_normalize_is_exactly_number() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::is_number::is_number;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = NormalizeFixture::default();
    let (number_type, any_type) = {
        let builtins = fixture.base.get_builtins();
        (builtins.numberType, builtins.anyType)
    };

    let number = fixture
        .normalize(number_type)
        .expect("expected normalized number");
    assert_eq!(is_number(number_type), number.is_exactly_number());

    let intersection = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![number_type, number_type],
    });
    let norm_intersection = fixture
        .normalize(intersection)
        .expect("expected normalized intersection");
    assert!(norm_intersection.is_exactly_number());

    let yoonion = fixture.arena.add_type(UnionType {
        options: alloc::vec![any_type, number_type],
    });
    let union_intersection = fixture
        .normalize(yoonion)
        .expect("expected normalized union");
    assert!(!union_intersection.is_exactly_number());
}
