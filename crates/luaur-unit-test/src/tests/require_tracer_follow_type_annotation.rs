//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:230:require_tracer_follow_type_annotation`
//! Source: `tests/RequireTracer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/RequireTracer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/RequireTracer.test.cpp
//! - outgoing:
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> method RequireTracerFixture::parse (tests/RequireTracer.test.cpp)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
//!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item require_tracer_follow_type_annotation

#[cfg(test)]
#[test]
fn require_tracer_follow_type_annotation() {
    use crate::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;
    use luaur_analysis::functions::trace_requires::trace_requires;
    use luaur_analysis::records::type_check_limits::TypeCheckLimits;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::rtti::ast_node_as;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
        r#"
        local R = game.Test :: (typeof(game.Redirect))
        require(R)
    "#,
    );
    unsafe {
        assert_eq!(2, (*block).body.size);
    }

    let result = trace_requires(
        &mut fixture.file_resolver.base,
        block,
        "ModuleName".to_string(),
        &TypeCheckLimits::default(),
    );

    unsafe {
        let local = ast_node_as::<AstStatLocal>(*(*block).body.data as *mut AstNode);
        assert!(!local.is_null());

        let value = *(*local).values.data as *mut AstNode;
        assert_eq!("game/Redirect", result.exprs.find(&value).unwrap().name);
    }
}
