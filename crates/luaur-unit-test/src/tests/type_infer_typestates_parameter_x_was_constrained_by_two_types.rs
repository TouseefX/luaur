//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_parameter_x_was_constrained_by_two_types() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x): string?
            local y: string | number = x
            return y
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(!result.errors.is_empty(), "{:?}", result.errors);

        let err = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
            .expect("expected TypeMismatch");
        assert_eq!("string?", to_string_type_id(err.wanted_type));
        assert_eq!("number | string", to_string_type_id(err.given_type));
        assert_eq!(
            "(number | string) -> string?",
            to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
        );
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "(string) -> string?",
            to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
        );
    }
}
