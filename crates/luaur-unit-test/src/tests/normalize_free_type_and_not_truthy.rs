//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1114:normalize_free_type_and_not_truthy`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method NormalizeFixture::getGlobalScope (tests/Normalize.test.cpp)
//!   - type_ref -> record NegationType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_free_type_and_not_truthy

#[cfg(test)]
#[test]
fn normalize_free_type_and_not_truthy() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::negation_type::NegationType;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = NormalizeFixture::default();

    let scope = fixture.get_global_scope();
    let builtins = fixture.base.builtin_types;
    let (free_ty, truthy_type) = unsafe {
        (
            fixture
                .arena
                .fresh_type_not_null_builtin_types_scope(&*builtins, scope),
            (*builtins).truthyType,
        )
    };
    let not_truthy = fixture.arena.add_type(NegationType::new(truthy_type));
    let intersection_ty = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![free_ty, not_truthy],
    });

    let norm = fixture
        .normalize(intersection_ty)
        .expect("expected normalized type");
    let result = fixture.type_from_normal(norm.as_ref());

    assert_eq!("'a & (false?)", to_string_type_id(result));
}
