//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1214:fragment_autocomplete_can_parse_complete_fragments`
//! Source: `tests/FragmentAutocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
//!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstExprBinary (Ast/include/Luau/Ast.h)
//!   - calls -> method BcInstHelper::op (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item fragment_autocomplete_can_parse_complete_fragments

#[cfg(test)]
#[test]
fn fragment_autocomplete_can_parse_complete_fragments() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use core::ffi::CStr;
    use luaur_ast::records::ast_expr::AstExpr;
    use luaur_ast::records::ast_expr_binary::{AstExprBinary, AstExprBinary_Op};
    use luaur_ast::records::ast_expr_local::AstExprLocal;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_ast::rtti::ast_node_as;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.check_with_options(&String::from(
        r#"
local x = 4
local y = 5
"#,
    ));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let fragment = fixture
        .base
        .parse_fragment(
            &String::from(
                r#"
local x = 4
local y = 5
local z = x + y
"#,
            ),
            &Position {
                line: 3,
                column: 15,
            },
            None,
        )
        .expect("expected fragment parse result");

    assert_eq!(
        Location {
            begin: Position { line: 3, column: 0 },
            end: Position {
                line: 3,
                column: 15
            },
        },
        unsafe { (*fragment.root).base.base.location }
    );
    assert_eq!("local z = x + y", fragment.fragment_to_parse);
    assert_eq!(4, fragment.ancestry.len());
    assert!(!fragment.root.is_null());
    assert_eq!(1, unsafe { (*fragment.root).body.size });

    let stat =
        unsafe { ast_node_as::<AstStatLocal>((*fragment.root).body.as_slice()[0] as *mut AstNode) };
    assert!(!stat.is_null());
    assert_eq!(1, unsafe { (*stat).vars.size });
    assert_eq!(1, unsafe { (*stat).values.size });
    assert_eq!("z", unsafe {
        CStr::from_ptr((*(*stat).vars.as_slice()[0]).name.value)
            .to_str()
            .unwrap()
    });

    let bin = unsafe { ast_node_as::<AstExprBinary>((*stat).values.as_slice()[0] as *mut AstNode) };
    assert!(!bin.is_null());
    assert_eq!(AstExprBinary_Op::Add, unsafe { (*bin).op });

    let lhs = unsafe { ast_node_as::<AstExprLocal>((*bin).left as *mut AstExpr as *mut AstNode) };
    let rhs = unsafe { ast_node_as::<AstExprLocal>((*bin).right as *mut AstExpr as *mut AstNode) };
    assert!(!lhs.is_null());
    assert!(!rhs.is_null());
    assert_eq!("x", unsafe {
        CStr::from_ptr((*(*lhs).local).name.value).to_str().unwrap()
    });
    assert_eq!("y", unsafe {
        CStr::from_ptr((*(*rhs).local).name.value).to_str().unwrap()
    });
}
