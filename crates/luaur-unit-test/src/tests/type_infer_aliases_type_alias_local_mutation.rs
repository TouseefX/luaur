//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_type_alias_local_mutation() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::table_type::TableType;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Cool = { a: number, b: string }
        local c: Cool = { a = 1, b = "s" }
        type NotCool<x> = Cool
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("c"));
    assert_eq!("Cool", to_string_type_id(ty));

    let ty = unsafe { follow_type_id(ty) };
    let ttv = unsafe { get_type_id::<TableType>(ty).as_ref() }.expect("expected TableType");
    assert!(ttv.instantiated_type_params.is_empty());
}
