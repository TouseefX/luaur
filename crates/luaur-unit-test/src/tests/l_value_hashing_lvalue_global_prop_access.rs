//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/LValue.test.cpp:146:l_value_hashing_lvalue_global_prop_access`
//! Source: `tests/LValue.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/LValue.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/LValue.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Field (Analysis/include/Luau/LValue.h)
//!   - type_ref -> record Symbol (Analysis/include/Luau/Symbol.h)
//!   - type_ref -> record AstName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record LValueHasher (Analysis/include/Luau/LValue.h)
//!   - type_ref -> type_alias RefinementMap (Analysis/include/Luau/LValue.h)
//!   - translates_to -> rust_item l_value_hashing_lvalue_global_prop_access

#[cfg(test)]
#[test]
fn l_value_hashing_lvalue_global_prop_access() {
    use luaur_analysis::records::builtin_types::BuiltinTypes;
    use luaur_analysis::records::field::Field;
    use luaur_analysis::records::l_value_hasher::LValueHasher;
    use luaur_analysis::records::symbol::Symbol;
    use luaur_analysis::type_aliases::l_value::LValue;
    use luaur_analysis::type_aliases::refinement_map::RefinementMap;
    use luaur_ast::records::ast_name::AstName;
    use std::ffi::CString;
    use std::sync::Arc;

    let t1 = CString::new("t").unwrap();
    let x1 = "x".to_string();
    let t_x1 = LValue::Field(Field {
        parent: Some(Arc::new(LValue::Symbol(Symbol::from_global(
            AstName::ast_name_c_char(t1.as_ptr()),
        )))),
        key: x1,
    });

    let t2 = CString::new("t").unwrap();
    let x2 = "x".to_string();
    let t_x2 = LValue::Field(Field {
        parent: Some(Arc::new(LValue::Symbol(Symbol::from_global(
            AstName::ast_name_c_char(t2.as_ptr()),
        )))),
        key: x2,
    });

    assert_eq!(t_x1, t_x1);
    assert_eq!(t_x1, t_x2);
    assert_eq!(t_x2, t_x2);

    let hasher = LValueHasher::default();
    assert_eq!(hasher.operator_call(&t_x1), hasher.operator_call(&t_x1));
    assert_eq!(hasher.operator_call(&t_x1), hasher.operator_call(&t_x2));
    assert_eq!(hasher.operator_call(&t_x2), hasher.operator_call(&t_x2));

    let builtin_types = BuiltinTypes::new();
    let mut m = RefinementMap::new();
    m.insert(t_x1, builtin_types.string_type());
    m.insert(t_x2, builtin_types.number_type());

    assert_eq!(1, m.len());
}
