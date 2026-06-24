//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:240:ast_json_encoder_encode_ast_expr_local`
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
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_local

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_local() {
    let mut local = AstLocal::new(
        ast_name(c"foo"),
        Location::default(),
        core::ptr::null_mut(),
        0,
        0,
        core::ptr::null_mut(),
        false,
    );
    let mut expr_local = AstExprLocal::new(Location::default(), &mut local, false);

    assert_eq!(
        json_ref(&mut expr_local),
        const2(
            r#"{"type":"AstExprLocal","location":"0,0 - 0,0","local":{"luauType":null,"name":"foo","isConst":false,"type":"AstLocal","location":"0,0 - 0,0"}}"#,
            r#"{"type":"AstExprLocal","location":"0,0 - 0,0","local":{"luauType":null,"name":"foo","type":"AstLocal","location":"0,0 - 0,0"}}"#,
        )
    );
}
