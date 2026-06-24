//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:247:generalization_a_number_string_string`
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
//!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item generalization_a_number_string_string

#[cfg(test)]
#[test]
fn generalization_a_number_string_string() {
    use crate::records::generalization_fixture::GeneralizationFixture;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = GeneralizationFixture::new();
    let (a_ty, a_free) = fixture.fresh_type();

    let upper_bound = fixture.arena.add_type(UnionType {
        options: vec![
            fixture.builtin_types.numberType,
            fixture.builtin_types.stringType,
        ],
    });
    unsafe {
        (*a_free).upper_bound = upper_bound;
    }

    let args = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[a_ty]);
    let rets = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.optionalStringType]);
    let fn_type = fixture
        .arena
        .add_type(FunctionType::function_type_new(args, rets, None, false));

    fixture.generalize(fn_type);

    assert_eq!(
        "(number | string) -> string?",
        fixture.to_string_type_id(fn_type)
    );
}
