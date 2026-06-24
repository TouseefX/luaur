//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_union_true_and_false() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x : boolean)
            local y1 : (true | false) = x -- OK
            local y2 : (true | false | (string & number)) = x -- OK
            local y3 : (true | (string & number) | false) = x -- OK
            local y4 : (true | (boolean & true) | false) = x -- OK
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
