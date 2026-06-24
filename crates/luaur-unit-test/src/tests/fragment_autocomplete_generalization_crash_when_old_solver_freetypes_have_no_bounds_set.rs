//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:2989:fragment_autocomplete_generalization_crash_when_old_solver_freetypes_have_no_bounds_set`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_generalization_crash_when_old_solver_freetypes_have_no_bounds_set() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
local UserInputService = game:GetService("UserInputService");

local Camera = workspace.CurrentCamera;

UserInputService.InputBegan:Connect(function(Input)
    if (Input.KeyCode == Enum.KeyCode.One) then
        local Up = Input.Foo
        local Vector = -(Up:Unit)
    end
end)
"#,
    );

    let dest = String::from(
        r#"
local UserInputService = game:GetService("UserInputService");

local Camera = workspace.CurrentCamera;

UserInputService.InputBegan:Connect(function(Input)
    if (Input.KeyCode == Enum.KeyCode.One) then
        local Up = Input.Foo
        local Vector = -(Up:Unit()) @1
    end
end)
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|_frag: &mut FragmentAutocompleteStatusResult| {}),
        None,
    );
}
