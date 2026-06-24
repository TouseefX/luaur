//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1229:type_infer_type_infer_recursion_limit_normalizer`
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
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method Fixture::validateErrors (tests/Fixture.cpp)
//!   - calls -> method Fixture::getErrors (tests/Fixture.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item type_infer_type_infer_recursion_limit_normalizer

#[cfg(test)]
#[test]
fn type_infer_type_infer_recursion_limit_normalizer() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::{FFlag, FInt};

    let _recursion_limit = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 10);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a,b,c,d,e,f,g,h,i,j>()
            local x : a&b&c&d&e&f&g&h&(i?)
            local y : (a&b&c&d&e&f&g&h&i)? = x
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let too_complex =
        "Code is too complex to typecheck! Consider simplifying the code around this area";

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(3, result.errors.len(), "{:?}", result.errors);
        let expected_locations = [
            Location {
                begin: Position {
                    line: 2,
                    column: 22,
                },
                end: Position {
                    line: 2,
                    column: 42,
                },
            },
            Location {
                begin: Position {
                    line: 3,
                    column: 22,
                },
                end: Position {
                    line: 3,
                    column: 42,
                },
            },
            Location {
                begin: Position {
                    line: 3,
                    column: 22,
                },
                end: Position {
                    line: 3,
                    column: 41,
                },
            },
        ];

        for (error, expected_location) in result.errors.iter().zip(expected_locations.iter()) {
            assert_eq!(*expected_location, error.location);
            assert_eq!(too_complex, to_string_type_error(error));
        }
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            Location {
                begin: Position {
                    line: 3,
                    column: 12,
                },
                end: Position {
                    line: 3,
                    column: 46,
                },
            },
            result.errors[0].location
        );
        assert_eq!(too_complex, to_string_type_error(&result.errors[0]));
    }
}
