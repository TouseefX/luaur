use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::functions::traverse_for_type_type_path::traverse_for_type;
use luaur_analysis::records::intersection_type::IntersectionType;

#[cfg(test)]
#[test]
fn subtyping_subtyping_reasonings_check_for_error_suppression_in_intersect_type_path() {
    let mut fixture = SubtypeFixture::default();
    let boolean_ty = fixture.builtin_types.booleanType;
    let number_ty = fixture.builtin_types.numberType;
    let error_ty = fixture.builtin_types.errorType;

    let sub_ty = boolean_ty;
    let super_ty = fixture.arena.add_type(IntersectionType {
        parts: vec![number_ty, error_ty],
    });
    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());

    for reasoning in result.reasoning().iter() {
        if reasoning.sub_path().path_empty() && reasoning.super_path().path_empty() {
            continue;
        }

        let opt_sub_leaf = traverse_for_type(
            sub_ty,
            reasoning.sub_path(),
            &fixture.builtin_types,
            &mut fixture.arena,
        );
        let opt_super_leaf = traverse_for_type(
            super_ty,
            reasoning.super_path(),
            &fixture.builtin_types,
            &mut fixture.arena,
        );

        assert!(opt_sub_leaf.is_some());
        assert!(opt_super_leaf.is_some());
        assert_eq!(Some(error_ty), opt_super_leaf);
    }
}
