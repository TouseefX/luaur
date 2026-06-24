//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:706:type_infer_operators_disallow_string_and_types_without_metatables_from_arithmetic_binary_ops`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method PathBuilder::mt (Analysis/src/TypePath.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_operators_disallow_string_and_types_without_metatables_from_arithmetic_binary_ops

#[cfg(test)]
#[test]
fn type_infer_operators_disallow_string_and_types_without_metatables_from_arithmetic_binary_ops() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::generic_error::GenericError;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_analysis::records::uninhabited_type_function::UninhabitedTypeFunction;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local a = "1.24" + 123 -- not allowed

        local foo = {
            value = 10
        }

        local b = foo + 1 -- not allowed

        local bar = {
            value = 1
        }

        local mt = {}

        setmetatable(bar, mt)

        mt.__add = function(a: typeof(bar), b: number): number
            return a.value + b
        end

        local c = bar + 1 -- allowed

        local d = bar + foo -- not allowed
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0])
            .expect("expected UninhabitedTypeFunction");
        assert_eq!(
            Location {
                begin: Position {
                    line: 2,
                    column: 18
                },
                end: Position {
                    line: 2,
                    column: 30
                }
            },
            result.errors[0].location
        );

        type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[1])
            .expect("expected UninhabitedTypeFunction");
        assert_eq!(
            Location {
                begin: Position {
                    line: 8,
                    column: 18
                },
                end: Position {
                    line: 8,
                    column: 25
                }
            },
            result.errors[1].location
        );

        type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[2])
            .expect("expected UninhabitedTypeFunction");
        assert_eq!(
            Location {
                begin: Position {
                    line: 24,
                    column: 18
                },
                end: Position {
                    line: 24,
                    column: 27
                }
            },
            result.errors[2].location
        );
    } else {
        let tm0 =
            type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
        assert_eq!("number", to_string_type_id(tm0.wanted_type));
        assert_eq!("string", to_string_type_id(tm0.given_type));

        let gen1 =
            type_error_data_ref::<GenericError>(&result.errors[1]).expect("expected GenericError");
        assert_eq!(
            "Binary operator '+' not supported by types 'foo' and 'number'",
            gen1.message()
        );

        let tm2 =
            type_error_data_ref::<TypeMismatch>(&result.errors[2]).expect("expected TypeMismatch");
        assert_eq!("number", to_string_type_id(tm2.wanted_type));
        assert_eq!(
            fixture.base.require_type_string(&String::from("foo")),
            tm2.given_type
        );
    }
}
