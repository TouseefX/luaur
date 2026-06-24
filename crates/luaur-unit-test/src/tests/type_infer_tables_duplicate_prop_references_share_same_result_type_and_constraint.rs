#[cfg(test)]
#[test]
fn type_infer_tables_duplicate_prop_references_share_same_result_type_and_constraint() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local tbl = {}
function f(x : number) : () end
function tbl:updateAmmoText()
    f(self.leadingZeros)
    local y = self.leadingZeros - 3
end
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
