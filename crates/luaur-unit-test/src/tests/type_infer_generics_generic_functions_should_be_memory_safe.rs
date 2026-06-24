//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:896:type_infer_generics_generic_functions_should_be_memory_safe`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_generics_generic_functions_should_be_memory_safe

#[cfg(test)]
#[test]
fn type_infer_generics_generic_functions_should_be_memory_safe() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
-- At one point this produced a UAF
type T<a> = { a: U<a>, b: a }
type U<a> = { c: T<a>?, d : a }
local x: T<number> = { a = { c = nil, d = 5 }, b = 37 }
x.a.c = x
local y: T<string> = { a = { c = nil, d = 5 }, b = 37 }
y.a.c = y
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);

        let mismatch1 =
            type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
        let mismatch2 =
            type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");

        assert_eq!(
            Location {
                begin: Position {
                    line: 7,
                    column: 42
                },
                end: Position {
                    line: 7,
                    column: 43
                },
            },
            result.errors[0].location
        );
        assert_eq!("number", to_string_type_id(mismatch1.given_type));
        assert_eq!("string", to_string_type_id(mismatch1.wanted_type));

        assert_eq!(
            Location {
                begin: Position {
                    line: 7,
                    column: 51
                },
                end: Position {
                    line: 7,
                    column: 53
                },
            },
            result.errors[1].location
        );
        assert_eq!("number", to_string_type_id(mismatch2.given_type));
        assert_eq!("string", to_string_type_id(mismatch2.wanted_type));
    } else {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        let expected = "Expected this to be exactly 'T<string>', but got 'y'\n\
caused by:\n  \
Property 'a' is not compatible.\n\
Expected this to be exactly 'U<string>', but got '{| c: T<string>?, d: number |}'\n\
caused by:\n  \
Property 'd' is not compatible.\n\
Expected this to be exactly 'string', but got 'number'";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
