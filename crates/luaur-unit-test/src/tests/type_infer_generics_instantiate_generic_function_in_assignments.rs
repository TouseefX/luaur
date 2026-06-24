//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1288:type_infer_generics_instantiate_generic_function_in_assignments`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_generics_instantiate_generic_function_in_assignments

#[cfg(test)]
#[test]
fn type_infer_generics_instantiate_generic_function_in_assignments() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(a, b)
            return a(b)
        end

        function bar()
            local c: ((number)->number, number)->number = foo -- no error
            c = foo -- no error
            local d: ((number)->number, string)->number = foo -- error from arg 2 (string) not being convertable to number from the call a(b)
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
        "((number) -> number, string) -> number",
        to_string_type_id(tm.wanted_type)
    );

    let expected_given =
        if FFlag::LuauInstantiateInSubtyping.get() || !FFlag::DebugLuauForceOldSolver.get() {
            "<a, b...>((a) -> (b...), a) -> (b...)"
        } else {
            "((number) -> number, number) -> number"
        };
    assert_eq!(expected_given, to_string_type_id(tm.given_type));
}
