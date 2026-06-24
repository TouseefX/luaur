//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Error.test.cpp:13:error_type_error_code_should_return_nonzero_code`
//! Source: `tests/Error.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Error.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/Error.test.cpp
//! - outgoing:
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item error_type_error_code_should_return_nonzero_code

#[cfg(test)]
#[test]
fn error_type_error_code_should_return_nonzero_code() {
    use luaur_analysis::records::type_error::TypeError;
    use luaur_analysis::records::unknown_symbol::{Context, UnknownSymbol};
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let e = TypeError::type_error_location_type_error_data(
        Location {
            begin: Position { line: 0, column: 0 },
            end: Position { line: 0, column: 1 },
        },
        UnknownSymbol::new("Foo".to_string(), Context::Binding).into(),
    );

    assert!(e.code() >= 1000);
}
