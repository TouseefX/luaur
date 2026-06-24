//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:3978:fragment_autocomplete_tagged_union_completion_second_branch_of_union_new_solver`
//! Source: `tests/FragmentAutocomplete.test.cpp`

#[cfg(test)]
#[test]
fn fragment_autocomplete_tagged_union_completion_second_branch_of_union_new_solver() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use luaur_analysis::records::fragment_autocomplete_status_result::FragmentAutocompleteStatusResult;
    use luaur_common::macros::luau_assert::LUAU_ASSERT;

    let source = String::from(
        r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then

end
"#,
    );

    let dest = String::from(
        r#"
type Ok<T> = { type: "ok", value: T}
type Err<E> = { type : "err", error : E}
type Result<T,E> = Ok<T> | Err<E>

local result = {} :: Result<number, string>

if result.type == "err" then
    result.@1
end
"#,
    );

    let mut fixture = FragmentAutocompleteFixture::default();
    fixture.base.autocomplete_fragment_in_new_solver(
        &source,
        &dest,
        '1',
        Box::new(|result: &mut FragmentAutocompleteStatusResult| {
            LUAU_ASSERT!(result.result.is_some());
            let ac = &result.result.as_ref().unwrap().ac_results;
            assert!(ac.entry_map.contains_key("type"));
            assert!(ac.entry_map.contains_key("error"));
        }),
        None,
    );
}
