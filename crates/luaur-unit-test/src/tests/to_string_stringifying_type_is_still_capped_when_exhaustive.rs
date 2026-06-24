//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_stringifying_type_is_still_capped_when_exhaustive() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f0() end
        function f1(f) return f or f0 end
        function f2(f) return f or f1 end
        function f3(f) return f or f2 end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let mut opts = ToStringOptions::default();
        opts.exhaustive = true;
        opts.max_type_length = 20;
        assert_eq!(
            "() -> ()",
            to_string_type_id_to_string_options(
                fixture.require_type_string(&String::from("f0")),
                &mut opts
            )
        );
        assert_eq!(
            "<a>(a) -> (() -> ()) ... *TRUNCATED*",
            to_string_type_id_to_string_options(
                fixture.require_type_string(&String::from("f1")),
                &mut opts
            )
        );
        assert_eq!(
            "<b>(b) -> (<a>(a) -> (() -> ())... *TRUNCATED*",
            to_string_type_id_to_string_options(
                fixture.require_type_string(&String::from("f2")),
                &mut opts
            )
        );
        assert_eq!(
            "<c>(c) -> (<b>(b) -> (<a>(a) -> (() -> ())... *TRUNCATED*",
            to_string_type_id_to_string_options(
                fixture.require_type_string(&String::from("f3")),
                &mut opts
            )
        );
    } else {
        let mut opts = ToStringOptions::default();
        opts.exhaustive = true;
        opts.max_type_length = 40;
        assert_eq!(
            "() -> ()",
            to_string_type_id_to_string_options(
                fixture.require_type_string(&String::from("f0")),
                &mut opts
            )
        );
        assert_eq!(
            "(() -> ()) -> () -> ()",
            to_string_type_id_to_string_options(
                fixture.require_type_string(&String::from("f1")),
                &mut opts
            )
        );
        assert_eq!(
            "((() -> ()) -> () -> ()) -> (() -> ()) -> ... *TRUNCATED*",
            to_string_type_id_to_string_options(
                fixture.require_type_string(&String::from("f2")),
                &mut opts
            )
        );
        assert_eq!(
            "(((() -> ()) -> () -> ()) -> (() -> ()) -> ... *TRUNCATED*",
            to_string_type_id_to_string_options(
                fixture.require_type_string(&String::from("f3")),
                &mut opts
            )
        );
    }
}
