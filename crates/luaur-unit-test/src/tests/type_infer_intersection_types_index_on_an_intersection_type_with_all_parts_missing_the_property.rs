//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_index_on_an_intersection_type_with_all_parts_missing_the_property()
{
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::unknown_property::UnknownProperty;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = {}
        type B = {}

        local function f(t: A & B)
            local x = t.x
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let up = unsafe { get_type_error::<UnknownProperty>(&result.errors[0]).as_ref() }
        .expect("expected UnknownProperty");
    assert_eq!("x", up.key());
}
