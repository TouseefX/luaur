//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_self_referential_type_alias() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type O = { x: number, incr: (O) -> number }
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let o_type = fixture
        .lookup_type(&String::from("O"))
        .expect("expected O alias");
    let o_type = unsafe { follow_type_id(o_type) };
    let o_table =
        unsafe { get_type_id::<TableType>(o_type).as_ref() }.expect("expected O table type");

    let incr = o_table.props.get("incr").expect("expected incr property");
    let incr_read_ty = incr.read_ty.expect("expected incr read type");

    let incr_func = unsafe { get_type_id::<FunctionType>(incr_read_ty).as_ref() }
        .expect("expected incr function type");
    let first_arg = first(incr_func.arg_types(), false).expect("expected first argument");

    assert_eq!(o_type, unsafe { follow_type_id(first_arg) });
}
