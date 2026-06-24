//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:674:simplify_intersect_parts_empty_table_non_empty`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item simplify_intersect_parts_empty_table_non_empty

#[cfg(test)]
#[test]
fn simplify_intersect_parts_empty_table_non_empty() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::simplify_intersection_simplify_alt_b::simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_ids::TypeIds;
    use luaur_analysis::records::union_type::UnionType;

    let mut base = Fixture::fixture_bool(false);
    base.get_frontend();
    let builtins = base.builtin_types;
    let mut arena = TypeArena::default();

    let mut empty = TableType::table_type();
    empty.state = TableState::Sealed;
    let empty_table = arena.add_type(empty);

    let number_type = unsafe { (*builtins).numberType };
    let string_type = unsafe { (*builtins).stringType };
    let mut non_empty = TableType::table_type();
    non_empty.props.insert(
        "p".to_string(),
        Property::rw_type_id(arena.add_type(UnionType {
            options: vec![number_type, string_type],
        })),
    );
    non_empty.state = TableState::Sealed;
    let non_empty_table = arena.add_type(non_empty);

    let mut parts = TypeIds::type_ids();
    parts.type_ids_initializer_list_type_id(&[non_empty_table, empty_table]);
    let result = simplify_intersection_not_null_builtin_types_not_null_type_arena_type_ids(
        builtins, &mut arena, parts,
    )
    .result;

    assert_eq!(
        "{ p: number | string }",
        to_string_type_id_to_string_options(result, &mut ToStringOptions::default())
    );
}
