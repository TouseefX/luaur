//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:298:type_infer_try_unify_free_tail_is_grown_properly`
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
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - translates_to -> rust_item type_infer_try_unify_free_tail_is_grown_properly

#[cfg(test)]
#[test]
fn type_infer_try_unify_free_tail_is_grown_properly() {
    use crate::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let number = fixture.get_builtins().numberType;

    let three_numbers = fixture.type_pack(vec![number, number, number]);
    let free_tail = fixture.free_type_pack();
    let number_and_free_tail = fixture.type_pack_with_tail(vec![number], free_tail);

    let errors = fixture.state.can_unify_type_pack_id_type_pack_id_bool(
        number_and_free_tail,
        three_numbers,
        false,
    );

    assert!(errors.is_empty(), "{:?}", errors);
}
