//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3719:fragment_autocomplete_not_null_assertion_caused_by_leaking_free_type_from_stale_module`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_not_null_assertion_caused_by_leaking_free_type_from_stale_module() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
local Players = game:GetService("Players")

Players.PlayerAdded:Connect(function(Player)
    for_,v in script.PlayerValue:GetChildren()do
        v
    end
end)
"#,
    );

    let dest = String::from(
        r#"
local Players = game:GetService("Players")

Players.PlayerAdded:Connect(function(Player)
    for_,v in script.PlayerValue:GetChildren()do
        v:L@1
    end
end)
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|_result: &mut FragmentAutocompleteStatusResult| {}),
        None,
    );
}
