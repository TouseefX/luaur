//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3648:fragment_autocomplete_nested_blocks_else_simple`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_nested_blocks_else_simple() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::enums::fragment_autocomplete_status::FragmentAutocompleteStatus;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
local function foo(t : {foo : string})
    local x = t.foo
    do
        if t then
        end
    end
end
"#,
    );
    let dest = String::from(
        r#"
local function foo(t : {foo : string})
    local x = t.foo
    do
        if t then
            x:@1
        end
    end
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|res: &mut FragmentAutocompleteStatusResult| {
            assert!(FragmentAutocompleteStatus::Success == res.status);
            assert!(res.result.is_some());
            let ac = &res.result.as_ref().unwrap().ac_results;
            assert!(!ac.entry_map.is_empty());
            assert!(ac.entry_map.contains_key("gsub"));
            assert!(ac.entry_map.contains_key("len"));
        }),
        None,
    );
}
