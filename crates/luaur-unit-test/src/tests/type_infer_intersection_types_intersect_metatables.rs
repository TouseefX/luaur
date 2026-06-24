//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_intersect_metatables() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    if !FFlag::DebugLuauForceOldSolver.get() {
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            function f(a: string?, b: string?)
                local x = setmetatable({}, { p = 5, q = a })
                local y = setmetatable({}, { q = b, r = "hi" })
                local z = setmetatable({}, { p = 5, q = nil, r = "hi" })

                type X = typeof(x)
                type Y = typeof(y)
                type Z = typeof(z)

                function g(xy: X&Y, yx: Y&X): (Z, Z)
                    return xy, yx
                end

                g(z, z)
            end
        "#,
            ),
            None,
        );

        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            local a : string? = nil
            local b : number? = nil

            local x = setmetatable({}, { p = 5, q = a });
            local y = setmetatable({}, { q = b, r = "hi" });
            local z = setmetatable({}, { p = 5, q = nil, r = "hi" });

            type X = typeof(x)
            type Y = typeof(y)
            type Z = typeof(z)

            local xy : X&Y = z;
            local yx : Y&X = z;
            z = xy;
            z = yx;
        "#,
            ),
            None,
        );

        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
}
