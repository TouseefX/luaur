//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_respect_partially_annotated_type_packs_2() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(): (number, boolean, string)
            return 42, true, "huh"
        end

        local a: number, b, c: string = f()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
}
