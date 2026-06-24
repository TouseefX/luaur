//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_type_alias_local_synthetic_mutation() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local c = { a = 1, b = "s" }
type Cool = typeof(c)
"#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("c"));
    let ty = unsafe { follow_type_id(ty) };
    let ttv = unsafe { get_type_id::<TableType>(ty).as_ref() }.expect("expected TableType");
    assert_eq!(Some(String::from("Cool")), ttv.name.clone());
}
