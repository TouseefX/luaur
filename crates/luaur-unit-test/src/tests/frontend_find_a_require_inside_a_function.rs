//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:108:frontend_find_a_require_inside_a_function`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - type_ref -> record NaiveFileResolver (tests/Frontend.test.cpp)
//!   - calls -> function traceRequires (Analysis/src/RequireTracer.cpp)
//!   - translates_to -> rust_item frontend_find_a_require_inside_a_function

#[cfg(test)]
#[test]
fn frontend_find_a_require_inside_a_function() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::records::naive_file_resolver::NaiveFileResolver;
    use luaur_analysis::functions::trace_requires::trace_requires;
    use luaur_analysis::records::type_check_limits::TypeCheckLimits;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };
    let program = fixture.base.base.parse(
        r#"
        function foo()
            local M = require(Modules.Foo.Bar)
        end
    "#,
        &ParseOptions::default(),
    );

    let mut naive_file_resolver = NaiveFileResolver::default();

    let result = trace_requires(
        &mut naive_file_resolver.base.base,
        program,
        String::new(),
        &TypeCheckLimits::default(),
    );
    assert_eq!(1, result.require_list.len());
}
