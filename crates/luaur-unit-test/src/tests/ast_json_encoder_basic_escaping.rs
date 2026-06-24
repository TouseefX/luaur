//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:86:ast_json_encoder_basic_escaping`
//! Source: `tests/AstJsonEncoder.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/AstJsonEncoder.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/AstJsonEncoder.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record AstArray (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item ast_json_encoder_basic_escaping

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_basic_escaping() {
    let mut bytes = "hello \"world\""
        .as_bytes()
        .iter()
        .map(|b| *b as core::ffi::c_char)
        .collect::<Vec<_>>();
    let mut s = AstExprConstantString::new(
        Location::default(),
        c_char_array(&mut bytes),
        AstExprConstantString::QuotedSimple,
    );

    assert_eq!(
        json_ref(&mut s),
        r#"{"type":"AstExprConstantString","location":"0,0 - 0,0","value":"hello \"world\""}"#
    );
}
