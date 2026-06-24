//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:242:type_infer_try_unify_variadic_type_pack_unification`
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
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
//!   - translates_to -> rust_item type_infer_try_unify_variadic_type_pack_unification

#[cfg(test)]
#[test]
fn type_infer_try_unify_variadic_type_pack_unification() {
    use crate::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let number = fixture.get_builtins().numberType;
    let string = fixture.get_builtins().stringType;

    let test_pack = fixture.type_pack(vec![number, string]);
    let variadic_pack = fixture.variadic_type_pack(number);

    fixture
        .state
        .try_unify_type_pack_id_type_pack_id_bool(test_pack, variadic_pack, false);

    assert!(fixture.state.failure);
    assert!(!fixture.state.errors.is_empty());
}
