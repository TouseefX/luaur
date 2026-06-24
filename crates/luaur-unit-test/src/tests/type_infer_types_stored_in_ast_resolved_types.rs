//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1298:type_infer_types_stored_in_ast_resolved_types`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - calls -> method Fixture::lookupType (tests/Fixture.cpp)
//!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_types_stored_in_ast_resolved_types

#[cfg(test)]
#[test]
fn type_infer_types_stored_in_ast_resolved_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::find_node_at_position_ast_query::find_node_at_position_source_module_position;
    use luaur_ast::records::ast_expr_function::AstExprFunction;
    use luaur_ast::records::position::Position;
    use luaur_ast::rtti::ast_node_as;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type alias = typeof("hello")
        local function foo(param: alias)
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let node = unsafe {
        find_node_at_position_source_module_position(
            &*fixture.get_main_source_module(),
            Position {
                line: 2,
                column: 16,
            },
        )
    };
    assert!(!node.is_null());

    let ty = fixture
        .lookup_type(&String::from("alias"))
        .expect("expected alias type");

    let func = unsafe { ast_node_as::<AstExprFunction>(node) };
    assert!(!func.is_null());
    assert_eq!(1, unsafe { (*func).args.len() });

    let arg = unsafe {
        *(*func)
            .args
            .as_slice()
            .first()
            .expect("expected function arg")
    };
    let annotation = unsafe { (*arg).annotation };
    assert!(!annotation.is_null());

    let module = unsafe { &*fixture.get_main_module(false) };
    assert_eq!(
        Some(&ty),
        module.ast_resolved_types.find(&(annotation as *const _))
    );
}
