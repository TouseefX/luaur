//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_cyclic_types_of_named_table_fields_do_not_expand_when_stringified() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        type Node = { Parent: Node?; }

        function f(node: Node)
            node.Parent = 1
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("Node?", to_string_type_id(tm.wanted_type));
    assert_eq!("number", to_string_type_id(tm.given_type));
}
