//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1905:frontend_parse_types`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method FrontendFixture::parseType (tests/Frontend.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record InternalCompilerError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias ErrorType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item frontend_parse_types

#[cfg(test)]
#[test]
fn frontend_parse_types() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::internal_error_reporter::InternalErrorReporter;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_check_limits::TypeCheckLimits;
    use luaur_analysis::type_aliases::error_type::ErrorType;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };
    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let mut arena = TypeArena::default();

    let mut parse_type = |fixture: &mut FrontendFixture, src: &str| {
        let mut ice_handler = InternalErrorReporter::default();
        fixture.get_frontend().parse_type(
            &mut allocator,
            &mut names,
            &mut ice_handler,
            TypeCheckLimits::default(),
            &mut arena,
            src,
        )
    };

    let ty1 = parse_type(&mut fixture, "(number, boolean?) -> string");
    assert_eq!("(number, boolean?) -> string", to_string_type_id(ty1));

    assert!(catch_unwind(AssertUnwindSafe(|| parse_type(
        &mut fixture,
        "illegal Luau Syntax here"
    )))
    .is_err());

    let ty3 = parse_type(&mut fixture, "blah<blahblah, number>");
    assert!(!unsafe { get_type_id::<ErrorType>(ty3) }.is_null());

    assert!(catch_unwind(AssertUnwindSafe(|| parse_type(
        &mut fixture,
        "number, boolean?) -> string"
    )))
    .is_err());
    assert!(catch_unwind(AssertUnwindSafe(|| parse_type(
        &mut fixture,
        "{size: number?"
    )))
    .is_err());
}
