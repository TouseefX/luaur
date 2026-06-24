#[cfg(test)]
#[test]
fn type_infer_generics_check_mutual_generic_functions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id1<a>(x:a):a
            local y: string = id2("hi")
            local z: number = id2(37)
            return x
        end

        function id2<a>(x:a):a
            local y: string = id1("hi")
            local z: number = id1(37)
            return x
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
