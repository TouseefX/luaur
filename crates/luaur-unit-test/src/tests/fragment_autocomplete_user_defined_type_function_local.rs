//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3205:fragment_autocomplete_user_defined_type_function_local`
//! Source: `tests/FragmentAutocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> function getOptions (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record FragmentAutocompleteStatusResult (Analysis/include/Luau/FragmentAutocomplete.h)
//!   - calls -> method FragmentAutocompleteFixtureImpl::autocompleteFragment (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> enum FragmentAutocompleteStatus (Analysis/include/Luau/FragmentAutocomplete.h)
//!   - translates_to -> rust_item fragment_autocomplete_user_defined_type_function_local

#[cfg(test)]
#[test]
fn fragment_autocomplete_user_defined_type_function_local() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let source = String::from(
        r#"--!strict
type function foo(x: type): type
    if x.tag == "singleton" then
        local t = x:value()

        return types.unionof(types.singleton(t), types.singleton(nil))
    end

    return types.number
end
"#,
    );

    let dest = String::from(
        r#"--!strict
type function foo(x: type): type
    if x.tag == "singleton" then
        local t = x:value()
        x
        return types.unionof(types.singleton(t), types.singleton(nil))
    end

    return types.number
end
"#,
    );

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_with_options(&source);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture
        .base
        .autocomplete_fragment(&dest, Position { line: 4, column: 9 }, None);
    assert_ne!(FragmentAutocompleteStatus::InternalIce, result.status);
}
