//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/OverloadResolver.test.cpp:261:overload_resolver_new_pass_table_with_indexer`
//! Source: `tests/OverloadResolver.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/OverloadResolver.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/OverloadResolver.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/UnifierSharedState.h
//! - incoming:
//!   - declares <- source_file tests/OverloadResolver.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - calls -> method OverloadResolverFixture::fn (tests/OverloadResolver.test.cpp)
//!   - type_ref -> record OverloadResolver (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolverFixture::mkResolver (tests/OverloadResolver.test.cpp)
//!   - type_ref -> record OverloadResolution (Analysis/include/Luau/OverloadResolver.h)
//!   - calls -> method OverloadResolver::resolveOverload (Analysis/src/OverloadResolver.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item overload_resolver_new_pass_table_with_indexer

use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use luaur_analysis::enums::table_state::TableState;
use luaur_analysis::records::table_indexer::TableIndexer;
use luaur_analysis::records::table_type::TableType;
use luaur_analysis::records::type_level::TypeLevel;
use luaur_analysis::type_aliases::props_type::Props;
use luaur_ast::records::location::Location;

#[cfg(test)]
#[test]
fn overload_resolver_new_pass_table_with_indexer() {
    let fixture = OverloadResolverFixture::new();

    let any_number_table = unsafe {
        (*fixture.arena).add_type(
            TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                &Props::new(),
                Some(TableIndexer {
                    index_type: fixture.builtin_types.anyType,
                    index_result_type: fixture.builtin_types.numberType,
                    is_read_only: false,
                }),
                TypeLevel::default(),
                fixture.root_scope.as_ref() as *const _ as *mut _,
                TableState::Sealed,
            ),
        )
    };

    let table_to_table = fixture.fn_item(&[any_number_table], &[any_number_table]);
    let mut resolver = fixture.mk_resolver();
    let args = fixture.pack_initializer_list_type_id(&[any_number_table]);
    let resolution = resolver.resolve_overload(
        table_to_table,
        args,
        Location::default(),
        fixture.empty_set,
        false,
    );

    assert_eq!(1, resolution.ok.len());
    assert_eq!(0, resolution.potential_overloads.len());
    assert_eq!(0, resolution.incompatible_overloads.len());
    assert_eq!(0, resolution.non_functions.len());
    assert_eq!(0, resolution.arity_mismatches.len());
}
