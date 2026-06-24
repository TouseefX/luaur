//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:353:type_infer_try_unify_fuzz_tail_unification_issue`
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
//!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
//!   - translates_to -> rust_item type_infer_try_unify_fuzz_tail_unification_issue

#[cfg(test)]
#[test]
fn type_infer_try_unify_fuzz_tail_unification_issue() {
    use crate::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let any = fixture.get_builtins().anyType;

    let variadic_any = fixture.variadic_type_pack(any);
    let pack_tmp = fixture.type_pack_with_tail(vec![any], variadic_any);
    let pack_sub = fixture.type_pack_with_tail(vec![any, any], pack_tmp);

    let free_ty = fixture.fresh_type();
    let free_tp = fixture.free_type_pack();
    let pack_super = fixture.type_pack_with_tail(vec![free_ty], free_tp);

    fixture
        .state
        .try_unify_type_pack_id_type_pack_id_bool(pack_sub, pack_super, false);
}
