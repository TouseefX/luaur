//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:319:type_infer_try_unify_cli_50320_follow_in_any_unification`
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
//!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - translates_to -> rust_item type_infer_try_unify_cli_50320_follow_in_any_unification

#[cfg(test)]
#[test]
fn type_infer_try_unify_cli_50320_follow_in_any_unification() {
    use crate::records::try_unify_fixture::TryUnifyFixture;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = TryUnifyFixture::new();

    let free = fixture.free_type_pack();
    let target = fixture.type_pack(Vec::new());
    let func = fixture
        .arena
        .add_type(FunctionType::function_type_new(free, free, None, false));
    let any = fixture.get_builtins().anyType;

    fixture
        .state
        .try_unify_type_pack_id_type_pack_id_bool(free, target, false);

    fixture
        .state
        .try_unify_type_id_type_id_bool_bool_literal_properties(func, any, false, false, None);

    assert!(!fixture.state.failure);
    assert!(
        fixture.state.errors.is_empty(),
        "{:?}",
        fixture.state.errors
    );
}
