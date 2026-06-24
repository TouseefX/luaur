//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_unifier_3_supertail_covariant_with_sub() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function fib(n)
            return n + fib(n)
        end
    "#,
        ),
        None,
    );

    assert_eq!(
        "<a>(a) -> t1 where t1 = add<a, t1>",
        to_string_type_id(fixture.require_type_string(&String::from("fib")))
    );
}
