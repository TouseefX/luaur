//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_oss_1547() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local rand = 0

        function a()
            rand = (rand % 4) + 1;
        end

        function b()
            rand = math.max(rand - 1, 0);
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let rand_ty = fixture
        .base
        .get_type(&String::from("rand"), false)
        .expect("expected rand type");
    assert_eq!("number", to_string_type_id(rand_ty));
}
