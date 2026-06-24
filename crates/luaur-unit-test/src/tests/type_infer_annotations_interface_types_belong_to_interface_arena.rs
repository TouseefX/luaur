//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_interface_types_belong_to_interface_arena() {
    use crate::functions::is_in_arena::is_in_arena;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type A = {field: number}

        local n: A = {field = 551}

        return {n=n}
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    let a = module
        .exported_type_bindings
        .get("A")
        .expect("expected exported type A");

    assert!(is_in_arena(a.r#type(), &module.interface_types));
    assert!(!is_in_arena(
        a.r#type(),
        fixture.get_frontend().globals.global_types_mut()
    ));

    let exports_type = first(module.return_type, true).expect("expected module return type");
    let exports_table =
        unsafe { get_type_id::<TableType>(exports_type).as_ref() }.expect("expected return table");

    let n = exports_table
        .props
        .get("n")
        .and_then(|prop| prop.read_ty)
        .expect("expected n property read type");

    assert!(is_in_arena(n, &module.interface_types));
}
