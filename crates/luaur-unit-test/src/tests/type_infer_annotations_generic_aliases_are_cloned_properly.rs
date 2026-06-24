//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_generic_aliases_are_cloned_properly() {
    use crate::functions::is_in_arena::is_in_arena;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type Array<T> = { [number]: T }
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    let array = module
        .exported_type_bindings
        .get("Array")
        .expect("expected exported type Array");

    assert_eq!(1, array.type_params().len());

    let array_table =
        unsafe { get_type_id::<TableType>(array.r#type()).as_ref() }.expect("expected table type");

    assert_eq!(0, array_table.props.len());
    let indexer = array_table
        .indexer
        .as_ref()
        .expect("expected table indexer");

    assert!(is_in_arena(array.r#type(), &module.interface_types));
    assert_eq!(array.type_params()[0].ty(), indexer.index_result_type);
}
