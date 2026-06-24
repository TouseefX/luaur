//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:942:pretty_printer_types_should_not_be_considered_cyclic_if_they_are_not_recursive`
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
//!   - translates_to -> rust_item pretty_printer_types_should_not_be_considered_cyclic_if_they_are_not_recursive

#[cfg(test)]
#[test]
fn pretty_printer_types_should_not_be_considered_cyclic_if_they_are_not_recursive() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::default();
    let code = String::from(
        r#"
        local common: {foo:string} = {foo = 'foo'}

        local t = {}
        t.x = common
        t.y = common
    "#,
    );
    let expected = String::from(
        r#"
        local common: {foo:string} = {foo = 'foo'}

        local t:{x:{foo:string},y:{foo:string}}={}
        t.x = common
        t.y = common
    "#,
    );

    assert_eq!(expected, fixture.decorate_with_types(&code));
}
