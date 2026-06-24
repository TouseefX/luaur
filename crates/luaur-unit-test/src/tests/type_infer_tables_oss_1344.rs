#[cfg(test)]
#[test]
fn type_infer_tables_oss_1344() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        type t = {
        	value: string?,
        }

        local t: t = {}

        if not t.value then
        	t.value = ""
        end

        local s: string? = nil

        if not s then
        	s = ""
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
