//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:62:type_infer_try_unify_incompatible_functions_are_preserved`
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
//!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_try_unify_incompatible_functions_are_preserved

#[cfg(test)]
#[test]
fn type_infer_try_unify_incompatible_functions_are_preserved() {
    use crate::records::try_unify_fixture::TryUnifyFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = TryUnifyFixture::default();
    let number_type = fixture.get_builtins().numberType;
    let string_type = fixture.get_builtins().stringType;
    let arg_one = fixture.fresh_type();
    let arg_two = fixture.fresh_type();
    let function_one = fixture.function_type(vec![arg_one], vec![number_type]);
    let function_one_saved = to_string_type_id(function_one);
    let function_two = fixture.function_type(vec![arg_two], vec![string_type]);
    let function_two_saved = to_string_type_id(function_two);

    fixture
        .state
        .try_unify_type_id_type_id_bool_bool_literal_properties(
            function_two,
            function_one,
            false,
            false,
            None,
        );

    assert!(fixture.state.failure);
    assert!(!fixture.state.errors.is_empty());
    assert_eq!(function_one_saved, to_string_type_id(function_one));
    assert_eq!(function_two_saved, to_string_type_id(function_two));
}
