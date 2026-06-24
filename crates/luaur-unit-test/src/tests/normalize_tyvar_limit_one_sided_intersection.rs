//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1149:normalize_tyvar_limit_one_sided_intersection`
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
//!   - calls -> method NormalizeFixture::getGlobalScope (tests/Normalize.test.cpp)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_tyvar_limit_one_sided_intersection

#[cfg(test)]
#[test]
fn normalize_tyvar_limit_one_sided_intersection() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use alloc::vec::Vec;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = NormalizeFixture::default();
    let scope = fixture.get_global_scope();
    let builtins = fixture.base.builtin_types;

    let mut options = Vec::new();
    for _ in 0..120 {
        options.push(unsafe {
            fixture
                .arena
                .fresh_type_not_null_builtin_types_scope(&*builtins, scope)
        });
    }

    let unknown_type = unsafe { (*builtins).unknownType };
    let union = fixture.arena.add_type(UnionType { options });
    let target = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![unknown_type, union],
    });

    let norm = fixture.normalize(target);
    assert!(
        norm.is_none(),
        "expected normalization to stop at the tyvar limit"
    );
}
