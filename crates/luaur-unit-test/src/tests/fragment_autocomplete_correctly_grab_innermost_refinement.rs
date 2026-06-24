//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4387:fragment_autocomplete_correctly_grab_innermost_refinement`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_correctly_grab_innermost_refinement() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
--!strict
type Type1 = { Type: "Type1", CommonKey: string, Type1Key: string }
type Type2 = { Type: "Type2", CommonKey: string, Type2Key: string }
type UnionType = Type1 | Type2

local foo: UnionType? = nil
if foo then
    if foo.Type == "Type2" then
    end
end
    "#,
    );

    let dest = String::from(
        r#"
--!strict
type Type1 = { Type: "Type1", CommonKey: string, Type1Key: string }
type Type2 = { Type: "Type2", CommonKey: string, Type2Key: string }
type UnionType = Type1 | Type2

local foo: UnionType? = nil
if foo then
    if foo.Type == "Type2" then
        foo.@1
    end
end
    "#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            let ac = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac.entry_map.is_empty());
            assert!(ac.entry_map.contains_key("Type2Key") as usize > 0);
            assert!(ac.entry_map.contains_key("Type1Key") as usize == 0);
        }),
        None,
    );
}
