//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:31:type_infer_try_unify_primitives_unify`
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
//!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_try_unify_primitives_unify

#[cfg(test)]
#[test]
fn type_infer_try_unify_primitives_unify() {
    use crate::records::try_unify_fixture::TryUnifyFixture;
    use luaur_analysis::records::primitive_type::PrimitiveType;

    let mut fixture = TryUnifyFixture::default();
    let number_one = fixture.arena.add_type(PrimitiveType {
        r#type: PrimitiveType::Number,
        metatable: None,
    });
    let number_two = fixture.arena.add_type(PrimitiveType {
        r#type: PrimitiveType::Number,
        metatable: None,
    });

    fixture
        .state
        .try_unify_type_id_type_id_bool_bool_literal_properties(
            number_two, number_one, false, false, None,
        );

    assert!(!fixture.state.failure);
    assert!(
        fixture.state.errors.is_empty(),
        "{:?}",
        fixture.state.errors
    );
}
