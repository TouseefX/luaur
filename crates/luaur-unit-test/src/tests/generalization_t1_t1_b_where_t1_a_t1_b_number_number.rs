//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:228:generalization_t1_t1_b_where_t1_a_t1_b_number_number`
//! Source: `tests/Generalization.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Generalization.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Generalization.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Generalization.test.cpp
//! - outgoing:
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - translates_to -> rust_item generalization_t1_t1_b_where_t1_a_t1_b_number_number

#[cfg(test)]
#[test]
fn generalization_t1_t1_b_where_t1_a_t1_b_number_number() {
    use crate::records::generalization_fixture::GeneralizationFixture;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::table_indexer::TableIndexer;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = GeneralizationFixture::new();
    let mut tt = TableType::table_type();
    tt.indexer = Some(TableIndexer {
        index_type: fixture.builtin_types.numberType,
        index_result_type: fixture.builtin_types.numberType,
        is_read_only: false,
    });
    let number_array = fixture.arena.add_type(tt);

    let (a_ty, a_free) = fixture.fresh_type();
    let (b_ty, b_free) = fixture.fresh_type();

    let upper_bound = fixture.arena.add_type(IntersectionType {
        parts: vec![b_ty, number_array, number_array],
    });
    unsafe {
        (*a_free).upper_bound = upper_bound;
        (*b_free).lower_bound = a_ty;
    }

    let args = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[a_ty, b_ty]);
    let function_ty = fixture.arena.add_type(FunctionType::function_type_new(
        args,
        fixture.builtin_types.emptyTypePack,
        None,
        false,
    ));

    fixture.generalize(function_ty);

    assert_eq!(
        "(unknown & {number}, unknown) -> ()",
        fixture.to_string_type_id(function_ty)
    );
}
