//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_detailed() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_detailed_to_string::to_string_detailed_type_id_to_string_options;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id3(a, b, c)
            return a, b, c
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::default();
    let id3_type = fixture.require_type_string(&String::from("id3"));
    let name_data = to_string_detailed_type_id_to_string_options(id3_type, &mut opts);

    assert_eq!(3, opts.name_map.types.size());
    assert_eq!("<a, b, c>(a, b, c) -> (a, b, c)", name_data.name);

    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(id3_type)).as_ref() }
        .expect("expected id3 to be a function type");
    let (params, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(3, params.len());

    assert_eq!(
        "a",
        to_string_type_id_to_string_options(params[0], &mut opts)
    );
    assert_eq!(
        "b",
        to_string_type_id_to_string_options(params[1], &mut opts)
    );
    assert_eq!(
        "c",
        to_string_type_id_to_string_options(params[2], &mut opts)
    );
}
