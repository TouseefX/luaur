//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_cloned_interface_maintains_pointers_between_definitions() {
    use crate::functions::is_in_arena::is_in_arena;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type Record = { name: string, location: string }
        local a: Record = { name="Waldo", location="?????" }
        local b: Record = { name="Santa Claus", location="Maui" } -- FIXME

        return {a=a, b=b}
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    let record_type = module
        .exported_type_bindings
        .get("Record")
        .expect("expected exported Record")
        .r#type();

    let exports_type = first(module.return_type, true).expect("expected module return type");
    let exports_table =
        unsafe { get_type_id::<TableType>(exports_type).as_ref() }.expect("expected return table");

    let a_type = exports_table
        .props
        .get("a")
        .and_then(|prop| prop.read_ty)
        .expect("expected a property read type");
    let b_type = exports_table
        .props
        .get("b")
        .and_then(|prop| prop.read_ty)
        .expect("expected b property read type");

    assert!(is_in_arena(record_type, &module.interface_types));
    assert!(is_in_arena(a_type, &module.interface_types));
    assert!(is_in_arena(b_type, &module.interface_types));

    let mut record_opts = ToStringOptions::to_string_options(true);
    let mut a_opts = ToStringOptions::to_string_options(true);
    let mut b_opts = ToStringOptions::to_string_options(true);
    let record_string = to_string_type_id_to_string_options(record_type, &mut record_opts);

    assert_eq!(
        record_string,
        to_string_type_id_to_string_options(a_type, &mut a_opts)
    );
    assert_eq!(
        record_string,
        to_string_type_id_to_string_options(b_type, &mut b_opts)
    );
}
