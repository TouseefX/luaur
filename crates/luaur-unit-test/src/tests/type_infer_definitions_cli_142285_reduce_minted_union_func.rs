//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:572:type_infer_definitions_cli_142285_reduce_minted_union_func`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotInferBinaryOperation (Analysis/include/Luau/Error.h)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record AstExprBinary (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item type_infer_definitions_cli_142285_reduce_minted_union_func

#[cfg(test)]
#[test]
fn type_infer_definitions_cli_142285_reduce_minted_union_func() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function middle(a: number, b: number): number
            return math.ceil((a + b) / 2 - 0.5)
        end

        local function find<T>(array: {T}, item: T): number?
            local l, m, r = 1, middle(1, #array), #array
            while l <= r do
                if item <= array[m] then
                    if item == array[m] then return m end
                    m, r = middle(l, m-1), m-1
                else
                    l, m = middle(m+1, r), m+1
                end
            end
        return nil
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = match &result.errors[0].data {
        TypeErrorData::CannotInferBinaryOperation(err) => err,
        other => panic!("expected CannotInferBinaryOperation, got {:?}", other),
    };
    assert_eq!(Some("item"), err.suggestedToAnnotate());
    assert_eq!(AstExprBinary_Op::CompareLe, err.op());
}
