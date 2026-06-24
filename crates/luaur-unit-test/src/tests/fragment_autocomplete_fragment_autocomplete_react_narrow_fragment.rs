//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4967:fragment_autocomplete_fragment_autocomplete_react_narrow_fragment`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_react_narrow_fragment() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;

    let src = String::from(
        "
        type React_Node = any
        type ReactElement<P, T> = any

        type React_StatelessFunctionalComponent<Props> = (props: Props, context: any) -> React_Node
        type React_Component<Props, State = nil> = {}
        type createElementFn = <P, T>(
            type_:
              | React_StatelessFunctionalComponent<P>
              | React_Component<P>
              | string,
            props: P?,
            ...(React_Node | (...any) -> React_Node)
        ) -> ReactElement<P, T>

        local createElement: createElementFn = nil :: any

        local function MyComponent(props: { foobar: string, barbaz: { bazquxx: string } })
        \treturn nil
        end

        createElement(MyComponent, { })
    ",
    );

    let dest = String::from(
        "
        type React_Node = any
        type ReactElement<P, T> = any

        type React_StatelessFunctionalComponent<Props> = (props: Props, context: any) -> React_Node
        type React_Component<Props, State = nil> = {}
        type createElementFn = <P, T>(
            type_:
              | React_StatelessFunctionalComponent<P>
              | React_Component<P>
              | string,
            props: P?,
            ...(React_Node | (...any) -> React_Node)
        ) -> ReactElement<P, T>

        local createElement: createElementFn = nil :: any

        local function MyComponent(props: { foobar: string, barbaz: { bazquxx: string } })
        \treturn nil
        end

        createElement(MyComponent, { f@1 })
    ",
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &src,
        &dest,
        '1',
        Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
            assert!(ac.result.is_some());
            let ac_results = &ac.result.as_ref().unwrap().ac_results;
            assert!(ac_results.entry_map.contains_key("foobar"));
        }),
        None,
    );
}
