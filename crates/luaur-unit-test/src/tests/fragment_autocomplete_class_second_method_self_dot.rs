//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5138:fragment_autocomplete_class_second_method_self_dot`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_class_second_method_self_dot() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
        r#"--!strict
class Bar
    public value: number
    function first(self)
    end
    function second(self)
    end
end
"#,
    );

    let dest = String::from(
        r#"--!strict
class Bar
    public value: number
    function first(self)
    end
    function second(self)
        self.@1
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            assert!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("value"));
        }),
        None,
    );
}
