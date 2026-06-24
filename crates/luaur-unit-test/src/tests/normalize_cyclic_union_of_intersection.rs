//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:881:normalize_cyclic_union_of_intersection`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_cyclic_union_of_intersection

#[cfg(test)]
#[test]
fn normalize_cyclic_union_of_intersection() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::blocked_type::BlockedType;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;

    let mut fixture = NormalizeFixture::default();
    let string_type = fixture.base.get_builtins().stringType;

    let bound_ty = fixture.arena.add_type(BlockedType::default());
    let intersect_ty = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![string_type, bound_ty],
    });
    let union_ty = fixture.arena.add_type(UnionType {
        options: alloc::vec![string_type, intersect_ty],
    });
    unsafe {
        (*as_mutable_type_id(bound_ty)).ty = TypeVariant::Bound(union_ty);
    }

    let nt = fixture
        .normalize(union_ty)
        .expect("expected normalized type");

    assert_eq!(
        "string",
        to_string_type_id(fixture.type_from_normal(nt.as_ref()))
    );
}
