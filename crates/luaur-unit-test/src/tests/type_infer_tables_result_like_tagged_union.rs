#[cfg(test)]
#[test]
fn type_infer_tables_result_like_tagged_union() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local function retry(func: (...any) -> ...any): { type: "ok", value: any } | { type: "failed" }
    local success: boolean, result: any = func()

    if success then
        return { type = "ok", value = result }
    else
        return { type = "failed" }
    end
end

return retry
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
