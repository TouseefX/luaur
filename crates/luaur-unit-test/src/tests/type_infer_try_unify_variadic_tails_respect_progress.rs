//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:252:type_infer_try_unify_variadic_tails_respect_progress`
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
//!   - translates_to -> rust_item type_infer_try_unify_variadic_tails_respect_progress

#[cfg(test)]
#[test]
fn type_infer_try_unify_variadic_tails_respect_progress() {
    use crate::records::try_unify_fixture::TryUnifyFixture;

    let mut fixture = TryUnifyFixture::new();
    let number = fixture.get_builtins().numberType;
    let string = fixture.get_builtins().stringType;
    let boolean = fixture.get_builtins().booleanType;

    let variadic_pack = fixture.variadic_type_pack(boolean);
    let a = fixture.type_pack(vec![number, string, boolean, boolean]);
    let b = fixture.type_pack_with_tail(vec![number, string], variadic_pack);

    fixture
        .state
        .try_unify_type_pack_id_type_pack_id_bool(b, a, false);

    assert!(!fixture.state.failure);
    assert!(fixture.state.errors.is_empty());
}
