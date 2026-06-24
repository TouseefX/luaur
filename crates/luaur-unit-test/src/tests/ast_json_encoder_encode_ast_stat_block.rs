//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:96:ast_json_encoder_encode_ast_stat_block`
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
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstArray (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_block

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_stat_block() {
    let mut astlocal = AstLocal::new(
        ast_name(c"a_local"),
        Location::default(),
        core::ptr::null_mut(),
        0,
        0,
        core::ptr::null_mut(),
        false,
    );
    let mut vars = [&mut astlocal as *mut AstLocal];
    let mut local = AstStatLocal::new(
        Location::default(),
        array(&mut vars),
        AstArray::default(),
        None,
        false,
    );
    let mut body = [&mut local as *mut AstStatLocal as *mut AstStat];
    let mut block = AstStatBlock::new(Location::default(), array(&mut body), true);

    assert_eq!(
        json_ref(&mut block),
        const2(
            r#"{"type":"AstStatBlock","location":"0,0 - 0,0","hasEnd":true,"body":[{"type":"AstStatLocal","location":"0,0 - 0,0","vars":[{"luauType":null,"name":"a_local","isConst":false,"type":"AstLocal","location":"0,0 - 0,0"}],"values":[]}]}"#,
            r#"{"type":"AstStatBlock","location":"0,0 - 0,0","hasEnd":true,"body":[{"type":"AstStatLocal","location":"0,0 - 0,0","vars":[{"luauType":null,"name":"a_local","type":"AstLocal","location":"0,0 - 0,0"}],"values":[]}]}"#,
        )
    );
}
