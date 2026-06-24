//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:218:generalization_a_a`
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
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - translates_to -> rust_item generalization_a_a

#[cfg(test)]
#[test]
fn generalization_a_a() {
    use crate::records::generalization_fixture::GeneralizationFixture;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = GeneralizationFixture::new();
    let free_ty = fixture.fresh_type().0;
    let args = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[free_ty]);
    let rets = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[free_ty]);
    let fn_ty = fixture
        .arena
        .add_type(FunctionType::function_type_new(args, rets, None, false));

    fixture.generalize(fn_ty);

    assert_eq!("<a>(a) -> a", fixture.to_string_type_id(fn_ty));
}
