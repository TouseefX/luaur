//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_intersect_metatable_with_table() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let source = if !FFlag::DebugLuauForceOldSolver.get() {
        r#"
            local x = setmetatable({ a = 5 }, { p = 5 })
            local z = setmetatable({ a = 5, b = "hi" }, { p = 5 })

            type X = typeof(x)
            type Y = { b : string }
            type Z = typeof(z)

            function f(xy: X&Y, yx: Y&X): (Z, Z)
                return xy, yx
            end

            f(z, z)
        "#
    } else {
        r#"
            local x = setmetatable({ a = 5 }, { p = 5 });
            local z = setmetatable({ a = 5, b = "hi" }, { p = 5 });

            type X = typeof(x)
            type Y = { b : string }
            type Z = typeof(z)

            -- TODO: once we have shape types, we should be able to initialize these with z
            local xy : X&Y;
            local yx : Y&X;
            z = xy;
            z = yx;
        "#
    };

    let result = fixture
        .base
        .check_string_optional_frontend_options(&String::from(source), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
