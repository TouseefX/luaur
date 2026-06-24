//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:57:visit_type_some_free_types_do_not_have_bounds`
//! Source: `tests/VisitType.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/VisitType.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/IterativeTypeVisitor.h
//! - incoming:
//!   - declares <- source_file tests/VisitType.test.cpp
//! - outgoing:
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - translates_to -> rust_item visit_type_some_free_types_do_not_have_bounds

#[cfg(test)]
#[test]
fn visit_type_some_free_types_do_not_have_bounds() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_level::TypeLevel;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let builtins = fixture.get_builtins();

    let t = Type::from(FreeType {
        level: TypeLevel::default(),
        lower_bound: builtins.neverType,
        upper_bound: builtins.unknownType,
        ..FreeType::default()
    });

    let _ = to_string_type_id(&t as *const Type);
}
