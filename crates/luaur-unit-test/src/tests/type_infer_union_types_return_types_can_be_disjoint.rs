//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_return_types_can_be_disjoint() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local count = 0
        function most_of_the_natural_numbers(): number?
            if count < 10 then
                count = count + 1
                return count
            else
                return nil
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let utv = unsafe {
        get_type_id::<FunctionType>(
            fixture.require_type_string(&String::from("most_of_the_natural_numbers")),
        )
        .as_ref()
    };
    assert!(utv.is_some());
}
