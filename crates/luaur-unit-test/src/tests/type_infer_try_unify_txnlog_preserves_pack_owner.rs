//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:342:type_infer_try_unify_txnlog_preserves_pack_owner`
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
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - translates_to -> rust_item type_infer_try_unify_txnlog_preserves_pack_owner

#[cfg(test)]
#[test]
fn type_infer_try_unify_txnlog_preserves_pack_owner() {
    use crate::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let a = fixture.free_type_pack();
    let b = fixture.get_builtins().anyTypePack;

    fixture
        .state
        .try_unify_type_pack_id_type_pack_id_bool(a, b, false);
    fixture.state.log.commit();

    assert_eq!(
        unsafe { (&*a).owning_arena() },
        &mut *fixture.arena as *mut _
    );
}
