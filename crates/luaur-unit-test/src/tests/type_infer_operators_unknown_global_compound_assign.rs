//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:933:type_infer_operators_unknown_global_compound_assign`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - translates_to -> rust_item type_infer_operators_unknown_global_compound_assign

#[cfg(test)]
#[test]
fn type_infer_operators_unknown_global_compound_assign() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    if FFlag::DebugLuauForceOldSolver.get() {
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
                --!nonstrict
                a = a + 1
                print(a)
            "#,
            ),
            None,
        );

        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Unknown global 'a'; consider assigning to it first",
            to_string_type_error(&result.errors[0])
        );
    }

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
            --!strict
            a += 1
            print(a)
        "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "Unknown global 'a'; consider assigning to it first",
        to_string_type_error(&result.errors[0])
    );

    if FFlag::DebugLuauForceOldSolver.get() {
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
                --!nonstrict
                a += 1
                print(a)
            "#,
            ),
            None,
        );

        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Unknown global 'a'; consider assigning to it first",
            to_string_type_error(&result.errors[0])
        );
    }
}
