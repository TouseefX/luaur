//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3461:type_infer_functions_overload_selection_bad_arity`
//! Source: `tests/TypeInfer.functions.test.cpp`

#[cfg(test)]
#[test]
fn type_infer_functions_overload_selection_bad_arity() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function foo<T>(f: ((number, number) -> "one") & T)
            local huh = f(42)
            local _ = huh
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_at_position_position(Position {
            line: 3,
            column: 23
        }))
    );
}
