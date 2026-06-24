//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:308:type_infer_try_unify_recursive_metatable_getmatchtag`
//! Source: `tests/TypeInfer.tryUnify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
//! - outgoing:
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_try_unify_recursive_metatable_getmatchtag

#[cfg(test)]
#[test]
fn type_infer_try_unify_recursive_metatable_getmatchtag() {
    use crate::records::try_unify_fixture::TryUnifyFixture;
    use luaur_analysis::records::metatable_type::MetatableType;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;

    let mut fixture = TryUnifyFixture::new();

    let redirect = fixture.fresh_type();
    let table = fixture.arena.add_type(TableType::table_type());
    let metatable = fixture.arena.add_type(MetatableType::new(redirect, table));

    unsafe {
        (*(redirect as *mut luaur_analysis::records::r#type::Type)).ty =
            TypeVariant::Bound(metatable);
    }

    let number = fixture.get_builtins().numberType;
    let variant = fixture.arena.add_type(UnionType {
        options: vec![metatable, number],
    });

    fixture
        .state
        .try_unify_type_id_type_id_bool_bool_literal_properties(
            metatable, variant, false, false, None,
        );
}
