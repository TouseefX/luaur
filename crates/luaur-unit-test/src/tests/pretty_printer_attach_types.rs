//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:907:pretty_printer_attach_types`
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
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - calls -> method Fixture::decorateWithTypes (tests/Fixture.cpp)
//!   - translates_to -> rust_item pretty_printer_attach_types

#[cfg(test)]
#[test]
fn pretty_printer_attach_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::default();
    let code = String::from(
        r#"
        local s='str'
        local t={a=1,b=false}
        local function fn()
            return 10
        end
    "#,
    );
    let expected = String::from(
        r#"
        local s:string='str'
        local t:{a:number,b:boolean}={a=1,b=false}
        local function fn(): number
            return 10
        end
    "#,
    );

    assert_eq!(expected, fixture.decorate_with_types(&code));
}
