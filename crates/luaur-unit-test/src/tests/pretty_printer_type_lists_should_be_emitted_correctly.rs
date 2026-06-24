//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:963:pretty_printer_type_lists_should_be_emitted_correctly`
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
//!   - calls -> method Fixture::decorateWithTypes (tests/Fixture.cpp)
//!   - translates_to -> rust_item pretty_printer_type_lists_should_be_emitted_correctly

#[cfg(test)]
#[test]
fn pretty_printer_type_lists_should_be_emitted_correctly() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::default();
    let code = String::from(
        r#"
        local a = function(a: string, b: number, ...: string): (string, ...number)
        end

        local b = function(...: string): ...number
        end

        local c = function()
        end
    "#,
    );
    let expected = String::from(
        r#"
        local a:(a:string,b:number,...string)->(string,...number)=function(a:string,b:number,...:string): (string,...number)
        end

        local b:(...string)->(...number)=function(...:string): ...number
        end

        local c:()->()=function(): ()
        end
    "#,
    );

    let actual = fixture.decorate_with_types(&code);

    assert_eq!(expected, actual);
}
