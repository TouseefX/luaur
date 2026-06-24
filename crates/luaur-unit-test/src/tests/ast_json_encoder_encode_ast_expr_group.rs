//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:193:ast_json_encoder_encode_ast_expr_group`
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
//!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstExprGroup (Ast/include/Luau/Ast.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_group

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_group() {
    let mut number =
        AstExprConstantNumber::new(Location::default(), 5.0, ConstantNumberParseResult::Ok);
    let mut group = AstExprGroup::new(
        Location::default(),
        &mut number as *mut AstExprConstantNumber as *mut AstExpr,
    );

    assert_eq!(
        json_ref(&mut group),
        r#"{"type":"AstExprGroup","location":"0,0 - 0,0","expr":{"type":"AstExprConstantNumber","location":"0,0 - 0,0","value":5}}"#
    );
}
