//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_oss_1561() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare class Vector3
            X: number
            Y: number
            Z: number
        end

        declare Vector3: {
            new: (number?, number?, number?) -> Vector3
        }
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local targetVelocity: Vector3 = Vector3.new()
        function set2D(X: number, Y: number)
            targetVelocity = Vector3.new(X, Y, targetVelocity.Z)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "(number, number) -> ()",
        to_string_type_id(fixture.require_type_string(&String::from("set2D")))
    );
}
