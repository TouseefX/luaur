//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_lookup_prop_of_intersection_containing_unions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::unknown_property::UnknownProperty;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function mergeOptions<T>(options: T & ({} | {}))
            return options.variables
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let unknown_prop = unsafe { get_type_error::<UnknownProperty>(&result.errors[0]).as_ref() }
        .expect("expected UnknownProperty");
    assert_eq!("variables", unknown_prop.key());
}
