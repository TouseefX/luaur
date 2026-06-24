//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:729:type_infer_builtins_bad_select_should_not_crash`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_builtins_bad_select_should_not_crash

#[cfg(test)]
#[test]
fn type_infer_builtins_bad_select_should_not_crash() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::count_mismatch::CountMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        do end
        local _ = function(l0,...)
        end
        local _ = function()
            _(_);
            _ += select(_())
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            Location {
                begin: Position {
                    line: 6,
                    column: 17,
                },
                end: Position {
                    line: 6,
                    column: 23,
                },
            },
            result.errors[0].location
        );
        let err = unsafe { get_type_error::<CountMismatch>(&result.errors[0]).as_ref() }
            .expect("expected CountMismatch");
        assert_eq!(1, err.expected());
        assert_eq!(0, err.actual());
    } else if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Argument count mismatch. Function expects at least 1 argument, but none are specified",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Argument count mismatch. Function expects at least 1 argument, but none are specified",
            to_string_type_error(&result.errors[1])
        );
    } else {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Argument count mismatch. Function '_' expects at least 1 argument, but none are specified",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Argument count mismatch. Function 'select' expects 1 argument, but none are specified",
            to_string_type_error(&result.errors[1])
        );
    }
}
