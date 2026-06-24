//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/RequireTracer.test.cpp:165:require_tracer_follow_typeof_in_return_type`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record RequireTraceResult (Analysis/include/Luau/RequireTracer.h)
//!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
//!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypePack (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeTypeof (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item require_tracer_follow_typeof_in_return_type

#[cfg(test)]
#[test]
fn require_tracer_follow_typeof_in_return_type() {
    use crate::methods::require_tracer_fixture_require_tracer_fixture::require_tracer_fixture_require_tracer_fixture;
    use luaur_analysis::functions::trace_requires::trace_requires;
    use luaur_analysis::records::type_check_limits::TypeCheckLimits;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
    use luaur_ast::records::ast_type_typeof::AstTypeTypeof;
    use luaur_ast::rtti::ast_node_as;
    use std::ffi::CStr;

    let mut fixture = require_tracer_fixture_require_tracer_fixture();
    let block = fixture.parse(
        r#"
        function foo(): typeof(require(workspace.CoolThing).UsefulObject)
        end
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
        let func = ast_node_as::<AstStatFunction>(*(*block).body.data as *mut AstNode);
        assert!(!func.is_null());

        let ret_annotation = (*(*func).func).return_annotation;
        assert!(!ret_annotation.is_null());

        let tp = ast_node_as::<AstTypePackExplicit>(ret_annotation as *mut AstNode);
        assert!(!tp.is_null());
        assert_eq!(1, (*tp).type_list.types.size);

        let typeof_annotation =
            ast_node_as::<AstTypeTypeof>(*(*tp).type_list.types.data as *mut AstNode);
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
