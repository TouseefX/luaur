//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_local_t_is_assigned_a_fresh_table_with_x_assigned_a_union_and_then_assert_restricts_actual_outflow_of_types(
) {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = nil

        if math.random() > 0.5 then
            t = {}
            t.x = if math.random() > 0.5 then 5 else "hello"
            assert(typeof(t.x) == "string")
        else
            t = {}
            t.x = if math.random() > 0.5 then 7 else true
            assert(typeof(t.x) == "boolean")
        end

        local x = t.x
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "boolean | number | string",
        to_string_type_id(fixture.base.base.require_type_string(&String::from("x")))
    );
}
