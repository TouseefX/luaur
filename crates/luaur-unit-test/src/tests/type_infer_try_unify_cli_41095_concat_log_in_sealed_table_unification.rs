//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:281:type_infer_try_unify_cli_41095_concat_log_in_sealed_table_unification`
//! Source: `tests/TypeInfer.tryUnify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_try_unify_cli_41095_concat_log_in_sealed_table_unification

#[cfg(test)]
#[test]
fn type_infer_try_unify_cli_41095_concat_log_in_sealed_table_unification() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        table.insert()
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "No overload for function accepts 0 arguments.",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!("MainModule", result.errors[1].module_name);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "Available overloads: <V>({V}, V) -> (); and <V>({V}, number, V) -> ()"
    } else {
        "Available overloads: ({'a}, 'a) -> (); and ({'a}, number, 'a) -> ()"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[1]));
}
