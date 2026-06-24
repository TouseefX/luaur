//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:630:type_infer_tc_after_error_recovery_no_replacement_name_in_error`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - calls -> method Frontend::setLuauSolverMode (Analysis/src/Frontend.cpp)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_tc_after_error_recovery_no_replacement_name_in_error

#[cfg(test)]
#[test]
fn type_infer_tc_after_error_recovery_no_replacement_name_in_error() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    {
        crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
        fixture
            .get_frontend()
            .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
                SolverMode::New
            } else {
                SolverMode::Old
            });
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            --!strict
            local t = { x = 10, y = 20 }
            return t.
        "#,
            ),
            None,
        );
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }

    {
        fixture
            .get_frontend()
            .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
                SolverMode::New
            } else {
                SolverMode::Old
            });
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            --!strict
            export type = number
            export type = string
        "#,
            ),
            None,
        );
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    }

    {
        crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
        fixture
            .get_frontend()
            .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
                SolverMode::New
            } else {
                SolverMode::Old
            });
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            --!strict
            function string.() end
        "#,
            ),
            None,
        );
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    }

    {
        fixture
            .get_frontend()
            .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
                SolverMode::New
            } else {
                SolverMode::Old
            });
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            --!strict
            local function () end
            local function () end
        "#,
            ),
            None,
        );
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    }

    {
        fixture
            .get_frontend()
            .set_luau_solver_mode(if !FFlag::DebugLuauForceOldSolver.get() {
                SolverMode::New
            } else {
                SolverMode::Old
            });
        let result = fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            --!strict
            local dm = {}
            function dm.() end
            function dm.() end
        "#,
            ),
            None,
        );
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    }
}
