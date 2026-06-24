//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:816:type_infer_operators_strict_binary_op_where_lhs_unknown`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_operators_strict_binary_op_where_lhs_unknown

#[cfg(test)]
#[test]
fn type_infer_operators_strict_binary_op_where_lhs_unknown() {
    use crate::records::fixture::Fixture;
    use alloc::format;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let ops = ["+", "-", "*", "/", "%", "^", ".."];
    let mut src = String::from("function foo(a, b)\n");

    for op in ops {
        src.push_str(&format!("local _ = a {} b\n", op));
    }

    src.push_str("end");

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(&src, None);

    assert_eq!(ops.len(), result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Operator '+' could not be applied to operands of types unknown and unknown; there is no corresponding overload for __add",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Operator '-' could not be applied to operands of types unknown and unknown; there is no corresponding overload for __sub",
            to_string_type_error(&result.errors[1])
        );
    } else {
        assert_eq!(
            "Unknown type used in + operation; consider adding a type annotation to 'a'",
            to_string_type_error(&result.errors[0])
        );
    }
}
