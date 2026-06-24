//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:60:ast_json_encoder_encode_constants`
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
//!   - type_ref -> record AstExprConstantNil (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstExprConstantBool (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstArray (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_constants

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_constants() {
    let mut nil = AstExprConstantNil::new(Location::default());
    let mut b = AstExprConstantBool::new(Location::default(), true);
    let mut n = AstExprConstantNumber::new(Location::default(), 8.2, ConstantNumberParseResult::Ok);
    let mut big_num = AstExprConstantNumber::new(
        Location::default(),
        0.1677721600000003,
        ConstantNumberParseResult::Ok,
    );
    let mut positive_infinity = AstExprConstantNumber::new(
        Location::default(),
        f64::INFINITY,
        ConstantNumberParseResult::Ok,
    );
    let mut negative_infinity = AstExprConstantNumber::new(
        Location::default(),
        f64::NEG_INFINITY,
        ConstantNumberParseResult::Ok,
    );
    let mut nan =
        AstExprConstantNumber::new(Location::default(), f64::NAN, ConstantNumberParseResult::Ok);

    let mut char_string = [
        b'a' as core::ffi::c_char,
        0x1d as core::ffi::c_char,
        0 as core::ffi::c_char,
        b'\\' as core::ffi::c_char,
        b'"' as core::ffi::c_char,
        b'b' as core::ffi::c_char,
    ];
    let mut needs_escaping = AstExprConstantString::new(
        Location::default(),
        c_char_array(&mut char_string),
        AstExprConstantString::QuotedSimple,
    );

    assert_eq!(
        json_ref(&mut nil),
        r#"{"type":"AstExprConstantNil","location":"0,0 - 0,0"}"#
    );
    assert_eq!(
        json_ref(&mut b),
        r#"{"type":"AstExprConstantBool","location":"0,0 - 0,0","value":true}"#
    );
    assert_eq!(
        json_ref(&mut n),
        r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":8.1999999999999993}"#
    );
    assert_eq!(
        json_ref(&mut big_num),
        r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":0.16777216000000031}"#
    );
    assert_eq!(
        json_ref(&mut positive_infinity),
        r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":Infinity}"#
    );
    assert_eq!(
        json_ref(&mut negative_infinity),
        r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":-Infinity}"#
    );
    assert_eq!(
        json_ref(&mut nan),
        r#"{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":NaN}"#
    );
    assert_eq!(
        json_ref(&mut needs_escaping),
        r#"{"type":"AstExprConstantString","location":"0,0 - 0,0","value":"a\u001d\u0000\\\"b"}"#
    );
}
