//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:485:to_dot_bound_table`
//! Source: `tests/ToDot.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ToDot.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToDot.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/ToDot.test.cpp
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - translates_to -> rust_item to_dot_bound_table

#[cfg(test)]
#[test]
fn to_dot_bound_table() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::to_dot_options::ToDotOptions;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::default();
    let number_type = fixture.get_builtins().numberType;
    let mut arena = TypeArena::default();

    let mut table = TableType::table_type();
    table
        .props
        .insert(String::from("x"), Property::rw_type_id(number_type));
    let ty = arena.add_type(table);

    let mut bound_table = TableType::table_type();
    bound_table.bound_to = Some(ty);
    let bound_ty = arena.add_type(bound_table);

    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"TableType 1\"];\nn1 -> n2 [label=\"boundTo\"];\nn2 [label=\"TableType 2\"];\nn2 -> n3 [label=\"x\"];\nn3 [label=\"number\"];\n}",
        to_dot(bound_ty, &opts)
    );
}
