#[cfg(test)]
#[test]
fn type_infer_generics_typefuns_sharing_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T<a> = { x: {a}, y: {number} }
        local o1: T<boolean> = { x = {true}, y = {5} }
        local x1, y1 = o1.x, o1.y
        local o2: T<string> = { x = {"hi"}, y = {37} }
        local x2, y2 = o2.x, o2.y
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let x1 = fixture.require_type_string(&String::from("x1"));
    let x2 = fixture.require_type_string(&String::from("x2"));
    let y1 = fixture.require_type_string(&String::from("y1"));
    let y2 = fixture.require_type_string(&String::from("y2"));

    assert_ne!(x1, x2);
    assert_eq!(y1, y2);
}
