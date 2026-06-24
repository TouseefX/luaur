//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:729:normalize_trivial_intersection_inhabited`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method NormalizeFixture::isInhabited (tests/Normalize.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item normalize_trivial_intersection_inhabited

#[cfg(test)]
#[test]
fn normalize_trivial_intersection_inhabited() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::intersection_type::IntersectionType;

    let mut fixture = NormalizeFixture::default();

    let (empty_type_pack, any_type_pack) = {
        let builtins = fixture.base.get_builtins();
        (builtins.emptyTypePack, builtins.anyTypePack)
    };

    let a = fixture.arena.add_type(FunctionType::function_type_new(
        empty_type_pack,
        any_type_pack,
        None,
        false,
    ));
    let c = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![a, a],
    });

    let n = fixture.normalize(c).expect("expected normalized type");
    assert!(fixture.is_inhabited(n.as_ref() as *const _));
}
