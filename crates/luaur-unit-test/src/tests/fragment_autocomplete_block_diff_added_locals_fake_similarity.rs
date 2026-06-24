//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3518:fragment_autocomplete_block_diff_added_locals_fake_similarity`
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
//!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
//!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
//!   - calls -> method FragmentAutocompleteFixtureImpl::parseHelper_ (tests/FragmentAutocomplete.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function blockDiffStart (Analysis/src/FragmentAutocomplete.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item fragment_autocomplete_block_diff_added_locals_fake_similarity

#[cfg(test)]
#[test]
fn fragment_autocomplete_block_diff_added_locals_fake_similarity() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::block_diff_start::block_diff_start;
    use luaur_analysis::records::source_module::SourceModule;
    use luaur_ast::records::position::Position;

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    let mut stale = SourceModule::source_module();
    let mut fresh = SourceModule::source_module();
    let old = fixture.base.parse_helper_(
        &mut stale,
        String::from(
            r#"local x = 4
local y = true
local z = 2 + 1"#,
        ),
    );
    let new = fixture.base.parse_helper_(
        &mut fresh,
        String::from(
            r#"local x = 4
local y = "tr"
local z = 3
local foo = 8"#,
        ),
    );

    let pos = block_diff_start(old.root, new.root, unsafe {
        (*new.root).body.as_slice()[2]
    });
    assert_eq!(Some(Position { line: 2, column: 0 }), pos);
}
