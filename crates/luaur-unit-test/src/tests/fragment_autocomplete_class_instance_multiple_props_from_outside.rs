//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:5243:fragment_autocomplete_class_instance_multiple_props_from_outside`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_class_instance_multiple_props_from_outside() {
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
class Point
    public x: number
    public y: number
    public z: number
end
local p = Point { x = 0, y = 0, z = 0 }
"#,
    );

    let dest = String::from(
        r#"--!strict
class Point
    public x: number
    public y: number
    public z: number
end
local p = Point { x = 0, y = 0, z = 0 }
p.@1
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
        &source,
        &dest,
        '1',
        Box::new(|frag: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(frag.result.is_some());
            let ac = &frag.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("x"));
            assert!(ac.entry_map.contains_key("y"));
            assert!(ac.entry_map.contains_key("z"));
        }),
        None,
    );
}
