//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4048:fragment_autocomplete_self_types_provide_rich_autocomplete`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_self_types_provide_rich_autocomplete() {
    use crate::records::fragment_autocomplete_builtins_fixture::FragmentAutocompleteBuiltinsFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let source = String::from(
        r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()

end
"#,
    );
    let dest = String::from(
        r#"
type Service = {
    Start: (self: Service) -> (),
    Prop: number
}

local Service: Service = {}

function Service:Start()
    self.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteBuiltinsFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            let ac_results = &result.result.as_ref().unwrap().ac_results;
            assert!(!ac_results.entry_map.is_empty());
            assert!(ac_results.entry_map.contains_key("Prop"));
            assert!(ac_results.entry_map.contains_key("Start"));
        }),
        None,
    );
}
