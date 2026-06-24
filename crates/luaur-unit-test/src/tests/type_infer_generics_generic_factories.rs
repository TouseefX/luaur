#[cfg(test)]
#[test]
fn type_infer_generics_generic_factories() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T<a> = { id: (a) -> a }
        type Factory = { build: <a>() -> T<a> }

        local f: Factory = {
            build = function<a>(): T<a>
                return {
                    id = function(x:a):a
                        return x
                    end
                }
            end
        }
        local y: string = f.build().id("hi")
        local z: number = f.build().id(37)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
