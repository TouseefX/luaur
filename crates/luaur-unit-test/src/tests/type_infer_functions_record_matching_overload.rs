//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1209:type_infer_functions_record_matching_overload`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_functions_record_matching_overload

#[cfg(test)]
#[test]
fn type_infer_functions_record_matching_overload() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::find_ast_ancestry_of_position_ast_query::find_ast_ancestry_of_position;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Overload = ((string) -> string) & ((number) -> number)
        local abc: Overload
        abc(1)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let source_module = fixture.get_main_source_module();
    let ancestry = unsafe {
        find_ast_ancestry_of_position(
            &*source_module,
            Position {
                line: 3,
                column: 10,
            },
            false,
        )
    };
    assert!(ancestry.len() >= 2, "ancestry was {:?}", ancestry);

    let parent_expr = ancestry[ancestry.len() - 2];
    assert!(
        luaur_ast::rtti::ast_node_is::<AstExprCall>(parent_expr),
        "expected AstExprCall"
    );

    let module = unsafe { &*fixture.get_main_module(false) };
    let overload = module
        .ast_overload_resolved_types
        .find(&(parent_expr as *const _))
        .expect("expected recorded overload type");
    assert_eq!("(number) -> number", to_string_type_id(*overload));
}
