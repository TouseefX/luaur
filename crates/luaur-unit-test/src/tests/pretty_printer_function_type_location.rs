//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:992:pretty_printer_function_type_location`
//! Source: `tests/PrettyPrinter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/PrettyPrinter.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Ast/include/Luau/PrettyPrinter.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/PrettyPrinter.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method Fixture::decorateWithTypes (tests/Fixture.cpp)
//!   - translates_to -> rust_item pretty_printer_function_type_location

#[cfg(test)]
#[test]
fn pretty_printer_function_type_location() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::default();
    let code = String::from(
        r#"
        local function foo(x: number): number
         return x
        end
        local g: (number)->number = foo
    "#,
    );
    let expected = String::from(
        r#"
        local function foo(x: number): number
         return x
        end
        local g: (number)->(number)=foo
    "#,
    );

    let actual = fixture.decorate_with_types(&code);

    assert_eq!(expected, actual);
}
