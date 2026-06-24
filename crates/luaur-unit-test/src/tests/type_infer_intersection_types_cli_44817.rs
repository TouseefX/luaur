//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_cli_44817() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type X = {x: number}
        type Y = {y: number}
        type Z = {z: number}

        type XY = {x: number, y: number}
        type XYZ = {x:number, y: number, z: number}

        function f(xy: XY, xyz: XYZ): (X&Y, X&Y&Z)
            return xy, xyz
        end

        local xNy, xNyNz = f({x = 0, y = 0}, {x = 0, y = 0, z = 0})

        local t1: XY = xNy -- Type 'X & Y' could not be converted into 'XY'
        local t2: XY = xNyNz -- Type 'X & Y & Z' could not be converted into 'XY'
        local t3: XYZ = xNyNz -- Type 'X & Y & Z' could not be converted into 'XYZ'
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
