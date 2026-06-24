//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1133:normalize_free_type_intersection_ordering`
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
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_free_type_intersection_ordering

#[cfg(test)]
#[test]
fn normalize_free_type_intersection_ordering() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = NormalizeFixture::default();

    let scope = fixture.get_global_scope();
    let builtins = fixture.base.builtin_types;
    let (free_ty, string_type) = unsafe {
        (
            fixture
                .arena
                .fresh_type_not_null_builtin_types_scope(&*builtins, scope),
            (*builtins).stringType,
        )
    };

    let order_a = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![free_ty, string_type],
    });
    let norm_a = fixture
        .normalize(order_a)
        .expect("expected normalized type");
    assert_eq!(
        "'a & string",
        to_string_type_id(fixture.type_from_normal(norm_a.as_ref()))
    );

    let order_b = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![string_type, free_ty],
    });
    let norm_b = fixture
        .normalize(order_b)
        .expect("expected normalized type");
    assert_eq!(
        "'a & string",
        to_string_type_id(fixture.type_from_normal(norm_b.as_ref()))
    );
}
