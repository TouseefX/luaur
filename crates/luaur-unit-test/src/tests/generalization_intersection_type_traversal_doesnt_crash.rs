//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:200:generalization_intersection_type_traversal_doesnt_crash`
//! Source: `tests/Generalization.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Generalization.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Generalization.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Generalization.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method Normalizer::intersectionType (Analysis/src/Normalize.cpp)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - translates_to -> rust_item generalization_intersection_type_traversal_doesnt_crash

#[cfg(test)]
#[test]
fn generalization_intersection_type_traversal_doesnt_crash() {
    use crate::records::generalization_fixture::GeneralizationFixture;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::scope::Scope;
    use std::sync::Arc;

    let mut fixture = GeneralizationFixture::new();
    let global_scope = Arc::as_ptr(&fixture.global_scope) as *mut Scope;

    let i = fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let h = fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let j = fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, global_scope);
    let intersection_type = fixture
        .arena
        .add_type(IntersectionType { parts: vec![h, j] });

    unsafe {
        (*get_mutable_type_id::<FreeType>(h)).upper_bound = i;
        (*get_mutable_type_id::<FreeType>(h)).lower_bound = fixture.builtin_types.neverType;
        (*get_mutable_type_id::<FreeType>(i)).upper_bound = fixture.builtin_types.unknownType;
        (*get_mutable_type_id::<FreeType>(i)).lower_bound = intersection_type;
        (*get_mutable_type_id::<FreeType>(j)).upper_bound = i;
        (*get_mutable_type_id::<FreeType>(j)).lower_bound = fixture.builtin_types.neverType;
    }

    fixture.generalize(intersection_type);
}
