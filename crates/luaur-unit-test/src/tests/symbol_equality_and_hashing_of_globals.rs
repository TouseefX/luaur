//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Symbol.test.cpp:15:symbol_equality_and_hashing_of_globals`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
//!   - translates_to -> rust_item symbol_equality_and_hashing_of_globals

#[cfg(test)]
#[test]
fn symbol_equality_and_hashing_of_globals() {
    use luaur_analysis::records::symbol::Symbol;
    use luaur_ast::records::ast_name::AstName;
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::ffi::CString;
    use std::hash::{Hash, Hasher};

    fn hash_symbol(symbol: &Symbol) -> u64 {
        let mut hasher = DefaultHasher::new();
        symbol.hash(&mut hasher);
        hasher.finish()
    }

    let s1 = CString::new("name").unwrap();
    let s2 = CString::new("name").unwrap();

    let one = AstName::ast_name_c_char(s1.as_ptr());
    let two = AstName::ast_name_c_char(s2.as_ptr());

    let n1 = Symbol::from_global(one);
    let n2 = Symbol::from_global(two);

    assert_eq!(n1, n1);
    assert_eq!(n1, n2);
    assert_eq!(n2, n2);

    assert_eq!(
        hash_symbol(&Symbol::from_global(one)),
        hash_symbol(&Symbol::from_global(one))
    );
    assert_eq!(
        hash_symbol(&Symbol::from_global(one)),
        hash_symbol(&Symbol::from_global(two))
    );
    assert_eq!(
        hash_symbol(&Symbol::from_global(two)),
        hash_symbol(&Symbol::from_global(two))
    );

    let mut the_map = HashMap::new();
    the_map.insert(n1, 5);
    the_map.insert(n2, 1);

    assert_eq!(1, the_map.len());
}
