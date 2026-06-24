//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/LValue.test.cpp:109:l_value_one_map_has_overlap_at_end_whereas_other_has_it_in_start`
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
//!   - type_ref -> type_alias RefinementMap (Analysis/include/Luau/LValue.h)
//!   - calls -> function mkSymbol (tests/LValue.test.cpp)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - calls -> function merge (tests/LValue.test.cpp)
//!   - translates_to -> rust_item l_value_one_map_has_overlap_at_end_whereas_other_has_it_in_start

#[cfg(test)]
#[test]
fn l_value_one_map_has_overlap_at_end_whereas_other_has_it_in_start() {
    use crate::functions::merge::merge;
    use crate::functions::mk_symbol::mk_symbol;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::builtin_types::BuiltinTypes;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::type_aliases::l_value::LValue;
    use luaur_analysis::type_aliases::refinement_map::RefinementMap;

    let builtin_types = BuiltinTypes::new();
    let mut m = RefinementMap::new();
    m.insert(LValue::Symbol(mk_symbol("a")), builtin_types.string_type());
    m.insert(LValue::Symbol(mk_symbol("b")), builtin_types.number_type());
    m.insert(LValue::Symbol(mk_symbol("c")), builtin_types.boolean_type());

    let mut other = RefinementMap::new();
    other.insert(LValue::Symbol(mk_symbol("c")), builtin_types.string_type());
    other.insert(LValue::Symbol(mk_symbol("d")), builtin_types.number_type());
    other.insert(LValue::Symbol(mk_symbol("e")), builtin_types.boolean_type());

    let mut arena = TypeArena::default();
    merge(&mut arena, &mut m, &other);

    assert_eq!(5, m.len());
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("a"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("b"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("c"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("d"))));
    assert!(m.contains_key(&LValue::Symbol(mk_symbol("e"))));

    assert_eq!(
        "string",
        to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("a"))).unwrap())
    );
    assert_eq!(
        "number",
        to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("b"))).unwrap())
    );
    assert_eq!(
        "boolean | string",
        to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("c"))).unwrap())
    );
    assert_eq!(
        "number",
        to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("d"))).unwrap())
    );
    assert_eq!(
        "boolean",
        to_string_type_id(*m.get(&LValue::Symbol(mk_symbol("e"))).unwrap())
    );
}
