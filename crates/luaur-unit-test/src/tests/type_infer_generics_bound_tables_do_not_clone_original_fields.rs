#[cfg(test)]
#[test]
fn type_infer_generics_bound_tables_do_not_clone_original_fields() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local exports = {}
local nested = {}

nested.name = function(t, k)
    local a = t.x.y
    return rawget(t, k)
end

exports.nested = nested
return exports
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
