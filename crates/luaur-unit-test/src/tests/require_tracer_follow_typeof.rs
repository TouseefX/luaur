//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:134:require_tracer_follow_typeof`
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
//!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
//!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstType (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeTypeof (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item require_tracer_follow_typeof

#[cfg(test)]
#[test]
fn require_tracer_follow_typeof() {
    use crate::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;
    use luaur_analysis::functions::trace_requires::trace_requires;
    use luaur_analysis::records::type_check_limits::TypeCheckLimits;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::ast_type_typeof::AstTypeTypeof;
    use luaur_ast::rtti::ast_node_as;
    use std::ffi::CStr;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
        r#"
        local R: typeof(require(workspace.CoolThing).UsefulObject)
    "#,
    );
    unsafe {
        assert_eq!(1, (*block).body.size);
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
        assert_eq!(1, (*local).vars.size);

        let ann = (*(*(*local).vars.data)).annotation;
        assert!(!ann.is_null());

        let typeof_annotation = ast_node_as::<AstTypeTypeof>(ann as *mut AstNode);
        assert!(!typeof_annotation.is_null());

        let index_name = ast_node_as::<AstExprIndexName>((*typeof_annotation).expr as *mut AstNode);
        assert!(!index_name.is_null());
        assert_eq!(
            "UsefulObject",
            CStr::from_ptr((*index_name).index.value).to_str().unwrap()
        );

        let call = ast_node_as::<AstExprCall>((*index_name).expr as *mut AstNode);
        assert!(!call.is_null());
        assert_eq!(1, (*call).args.size);

        let arg = *(*call).args.data as *mut AstNode;
        assert_eq!("workspace/CoolThing", result.exprs.find(&arg).unwrap().name);
    }
}
