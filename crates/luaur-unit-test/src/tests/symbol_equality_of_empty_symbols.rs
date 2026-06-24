//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Symbol.test.cpp:69:symbol_equality_of_empty_symbols`
//! Source: `tests/Symbol.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Symbol.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/Symbol.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
//!   - translates_to -> rust_item symbol_equality_of_empty_symbols

#[cfg(test)]
#[test]
fn symbol_equality_of_empty_symbols() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::records::symbol::Symbol;
    use luaur_ast::records::ast_local::AstLocal;
    use luaur_ast::records::ast_name::AstName;
    use luaur_ast::records::location::Location;
    use luaur_common::FFlag;
    use std::ffi::CString;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let s1 = CString::new("name").unwrap();
    let s2 = CString::new("name").unwrap();

    let one = AstName::ast_name_c_char(s1.as_ptr());
    let mut two = AstLocal::new(
        AstName::ast_name_c_char(s2.as_ptr()),
        Location::default(),
        core::ptr::null_mut(),
        0,
        0,
        core::ptr::null_mut(),
        false,
    );

    let global = Symbol::from_global(one);
    let local = Symbol::from_local(&mut two);
    let empty1 = Symbol::default();
    let empty2 = Symbol::default();

    assert_ne!(empty1, global);
    assert_ne!(empty1, local);
    assert_eq!(empty1, empty2);
}
