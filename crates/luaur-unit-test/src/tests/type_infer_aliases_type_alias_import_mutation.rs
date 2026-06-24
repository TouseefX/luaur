//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_type_alias_import_mutation() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_global_binding::get_global_binding;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture
        .base
        .check_string_optional_frontend_options(&String::from("type t10<x> = typeof(table)"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = get_global_binding(&mut fixture.get_frontend().globals, "table");

    assert_eq!("typeof(table)", to_string_type_id(ty));

    let ttv = unsafe { get_type_id::<TableType>(ty).as_ref() }.expect("expected table type");
    assert!(ttv.instantiated_type_params.is_empty());
}
