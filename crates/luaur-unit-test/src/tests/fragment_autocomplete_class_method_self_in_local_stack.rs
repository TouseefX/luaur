//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1105:fragment_autocomplete_class_method_self_in_local_stack`
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
//!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item fragment_autocomplete_class_method_self_in_local_stack

#[cfg(test)]
#[test]
fn fragment_autocomplete_class_method_self_in_local_stack() {
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
    public value: number
    function doThing(self)
    end
end
"#,
        ),
        &Position { line: 4, column: 2 },
    );

    assert_eq!(1, result.localStack.len());
    assert_eq!(result.localMap.size(), result.localStack.len());
    let last = *result.localStack.last().unwrap();
    assert_eq!(
        "self",
        unsafe { CStr::from_ptr((*last).name.value) }
            .to_str()
            .unwrap()
    );
}
