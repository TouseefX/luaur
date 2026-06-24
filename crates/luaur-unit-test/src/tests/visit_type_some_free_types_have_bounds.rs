//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/VisitType.test.cpp:64:visit_type_some_free_types_have_bounds`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item visit_type_some_free_types_have_bounds

#[cfg(test)]
#[test]
fn visit_type_some_free_types_have_bounds() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::scope::Scope;

    let _sff = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();
    let builtins = fixture.get_builtins();
    let mut scope = Scope::scope_type_pack_id(builtins.anyTypePack);

    let t = Type::from(FreeType::free_type_scope_type_id_type_id_polarity(
        &mut scope,
        builtins.neverType,
        builtins.numberType,
        Polarity::Unknown,
    ));

    assert_eq!("('a <: number)", to_string_type_id(&t as *const Type));
}
