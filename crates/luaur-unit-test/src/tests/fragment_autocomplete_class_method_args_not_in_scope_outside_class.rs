//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1125:fragment_autocomplete_class_method_args_not_in_scope_outside_class`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item fragment_autocomplete_class_method_args_not_in_scope_outside_class

#[cfg(test)]
#[test]
fn fragment_autocomplete_class_method_args_not_in_scope_outside_class() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use core::ffi::CStr;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
        &String::from(
            r#"
class Bar
    function method(self)
    end
end
local x = 4
"#,
        ),
        &Position {
            line: 6,
            column: 10,
        },
    );

    assert!(!result
        .localMap
        .iter()
        .any(|(name, _)| { unsafe { CStr::from_ptr(name.value) }.to_bytes() == b"self" }));
}
