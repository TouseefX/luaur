//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_intersect_metatable_subtypes() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x = setmetatable({ a = 5 }, { p = 5 })
        local y = setmetatable({ b = "hi" }, { p = 5, q = "hi" })
        local z = setmetatable({ a = 5, b = "hi" }, { p = 5, q = "hi" })

        type X = typeof(x)
        type Y = typeof(y)
        type Z = typeof(z)

        function f(xy: X&Y, yx: Y&X): (Z, Z)
            return xy, yx
        end

        f(z, z)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
