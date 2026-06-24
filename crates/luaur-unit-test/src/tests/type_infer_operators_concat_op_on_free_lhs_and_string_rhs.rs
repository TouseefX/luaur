//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:780:type_infer_operators_concat_op_on_free_lhs_and_string_rhs`
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
//!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotInferBinaryOperation (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_operators_concat_op_on_free_lhs_and_string_rhs

#[cfg(test)]
#[test]
fn type_infer_operators_concat_op_on_free_lhs_and_string_rhs() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::cannot_infer_binary_operation::CannotInferBinaryOperation;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(x)
            return x .. "y"
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "<a>(a) -> concat<a, string>",
            to_string_type_id(fixture.require_type_string(&String::from("f")))
        );
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        type_error_data_ref::<CannotInferBinaryOperation>(&result.errors[0])
            .expect("expected CannotInferBinaryOperation");
    }
}
