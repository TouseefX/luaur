//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_cli_80596_simplify_more_realistic_intersections() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _dcr = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = {
            x: number?,
            y: string?,
        }

        type B = {
            x: number?,
            z: string?,
        }

        type C = A & B

        function f(obj: C): number
            return obj.x or 3
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
