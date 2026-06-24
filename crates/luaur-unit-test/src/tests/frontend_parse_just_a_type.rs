//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1888:frontend_parse_just_a_type`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record Allocator (Ast/include/Luau/Allocator.h)
//!   - type_ref -> record AstNameTable (Ast/include/Luau/Lexer.h)
//!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeCheckLimits (Analysis/include/Luau/TypeCheckLimits.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method FrontendFixture::parseType (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item frontend_parse_just_a_type

#[cfg(test)]
#[test]
fn frontend_parse_just_a_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::builtin_types::BuiltinTypes;
    use luaur_analysis::records::internal_error_reporter::InternalErrorReporter;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_check_limits::TypeCheckLimits;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;

    let src = "(number, string) -> boolean?";

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    let mut arena = TypeArena::default();
    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let _builtin_types = BuiltinTypes::builtin_types();
    let mut ice_handler = InternalErrorReporter::default();
    let limits = TypeCheckLimits::default();

    let ty = fixture.get_frontend().parse_type(
        &mut allocator,
        &mut names,
        &mut ice_handler,
        limits,
        &mut arena,
        src,
    );

    assert_eq!("(number, string) -> boolean?", to_string_type_id(ty));
}
