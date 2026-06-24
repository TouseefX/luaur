//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5355:fragment_autocomplete_class_autocomplete_between_definitions`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_class_autocomplete_between_definitions() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
        r#"--!strict
class Bar
    public value: number
    function printvalue(self)
        print(self.value)
    end
end
"#,
    );

    let dest = String::from(
        r#"--!strict
class Bar
    public value: number
    function printvalue(self)
        print(self.value)
    end
    s@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            assert!(!frag.result.as_ref().unwrap().ac_results.entry_map.contains_key("self"));
        }),
        None,
    );
}
