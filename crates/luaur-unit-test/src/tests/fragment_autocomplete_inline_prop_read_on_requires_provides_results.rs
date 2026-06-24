//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4017:fragment_autocomplete_inline_prop_read_on_requires_provides_results`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_inline_prop_read_on_requires_provides_results() {
    use crate::functions::get_options::get_options;
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::string::String;
    use luaur_ast::records::position::Position;

    let module_a = String::from(
        r#"
local mod = { prop1 = true}
mod.prop2 = "a"
function mod.foo(a: number)
    return a
end
return mod
"#,
    );

    let main_module = String::from(
        r#"

"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture
        .base
        .base
        .base
        .file_resolver
        .source
        .insert(String::from("MainModule"), main_module);
    fixture
        .base
        .base
        .base
        .file_resolver
        .source
        .insert(String::from("MainModule/A"), module_a);
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("MainModule/A"), Some(get_options()));
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("MainModule"), Some(get_options()));

    let updated_main = String::from(
        r#"
require(script.A).
"#,
    );

    let result = fixture.base.autocomplete_fragment(
        &updated_main,
        Position {
            line: 1,
            column: 18,
        },
        None,
    );
    let ac_results = &result.result.as_ref().unwrap().ac_results;
    assert!(!ac_results.entry_map.is_empty());
    assert!(ac_results.entry_map.contains_key("prop1"));
    assert!(ac_results.entry_map.contains_key("prop2"));
    assert!(ac_results.entry_map.contains_key("foo"));
}
