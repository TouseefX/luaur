//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:4882:fragment_autocomplete_fragment_autocomplete_react_properties`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_fragment_autocomplete_react_properties() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let src = String::from(
        r#"
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
        	return nil
        end

    "#,
    );

    let dest = String::from(
        r#"
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
        	return nil
        end

        createElement(MyComponent, { f@1 })
        createElement(MyComponent, { barbaz = { b@2 } })
        createElement(MyComponent, { foobar = {}, b@3 })
    "#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_both_solvers(
        &src,
        &dest,
        '1',
        Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(ac.result.is_some());
            assert!(
                (ac.result
                    .as_ref()
                    .unwrap()
                    .ac_results
                    .entry_map
                    .contains_key("foobar") as usize)
                    > 0
            );
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &src,
        &dest,
        '2',
        Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(ac.result.is_some());
            assert!(
                (ac.result
                    .as_ref()
                    .unwrap()
                    .ac_results
                    .entry_map
                    .contains_key("bazquxx") as usize)
                    > 0
            );
        }),
        None,
    );

    fixture.base.autocomplete_fragment_in_both_solvers(
        &src,
        &dest,
        '3',
        Box::new(|ac: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(ac.result.is_some());
            assert!(
                (ac.result
                    .as_ref()
                    .unwrap()
                    .ac_results
                    .entry_map
                    .contains_key("barbaz") as usize)
                    > 0
            );
        }),
        None,
    );
}
