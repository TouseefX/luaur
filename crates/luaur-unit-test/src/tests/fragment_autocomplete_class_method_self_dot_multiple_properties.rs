//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5065:fragment_autocomplete_class_method_self_dot_multiple_properties`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_class_method_self_dot_multiple_properties() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let source = String::from(
        r#"--!strict
class Vec3
    public x: number
    public y: number
    public z: number
    function length(self)
    end
end
"#,
    );

    let dest = String::from(
        r#"--!strict
class Vec3
    public x: number
    public y: number
    public z: number
    function length(self)
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
            assert!(ac.entry_map.contains_key("x"));
            assert!(ac.entry_map.contains_key("y"));
            assert!(ac.entry_map.contains_key("z"));
        }),
        None,
    );
}
