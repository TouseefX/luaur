#[cfg(test)]
#[test]
fn type_infer_generics_instantiation_sharing_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(z)
          local o = {}
          o.x = o
          o.y = {5}
          o.z = z
          return o
        end
        local o1 = f(true)
        local x1, y1, z1 = o1.x, o1.y, o1.z
        local o2 = f("hi")
        local x2, y2, z2 = o2.x, o2.y, o2.z
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let x1 = fixture.require_type_string(&String::from("x1"));
    let x2 = fixture.require_type_string(&String::from("x2"));
    let y1 = fixture.require_type_string(&String::from("y1"));
    let y2 = fixture.require_type_string(&String::from("y2"));
    let z1 = fixture.require_type_string(&String::from("z1"));
    let z2 = fixture.require_type_string(&String::from("z2"));

    assert_ne!(x1, x2);
    assert_eq!(y1, y2);
    assert_ne!(z1, z2);
}
