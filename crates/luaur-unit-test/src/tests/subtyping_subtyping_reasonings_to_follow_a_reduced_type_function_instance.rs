use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::functions::traverse_type_path_alt_c::traverse;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;
use luaur_analysis::records::union_type::UnionType;

#[cfg(test)]
#[test]
fn subtyping_subtyping_reasonings_to_follow_a_reduced_type_function_instance() {
    let mut fixture = SubtypeFixture::default();
    let boolean_ty = fixture.builtin_types.booleanType;
    let buffer_ty = fixture.builtin_types.bufferType;
    let extern_ty = fixture.builtin_types.externType;
    let function_ty = fixture.builtin_types.functionType;
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;
    let table_ty = fixture.builtin_types.tableType;
    let thread_ty = fixture.builtin_types.threadType;
    let unknown_ty = fixture.builtin_types.unknownType;
    let never_ty = fixture.builtin_types.neverType;

    let long_ty = fixture.arena.add_type(UnionType {
        options: vec![
            boolean_ty,
            buffer_ty,
            extern_ty,
            function_ty,
            number_ty,
            string_ty,
            table_ty,
            thread_ty,
        ],
    });
    let tbl_ty = fixture.tbl(SubtypeFixture::props(vec![(
        "depth",
        Property::rw_type_id(unknown_ty),
    )]));
    let combined = fixture.meet(long_ty, tbl_ty);
    let sub_ty = fixture.arena.add_type(
        TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
            &fixture.builtin_types.typeFunctions.union_func,
            vec![combined, never_ty],
        ),
    );
    let super_ty = never_ty;

    let result = fixture.is_subtype_type_id_type_id(sub_ty, super_ty);
    assert!(!result.is_subtype());

    for reasoning in result.reasoning().iter() {
        if reasoning.sub_path().path_empty() && reasoning.super_path().path_empty() {
            continue;
        }

        let opt_sub_leaf = traverse(
            sub_ty,
            reasoning.sub_path(),
            &fixture.builtin_types,
            &mut fixture.arena,
        );
        let opt_super_leaf = traverse(
            super_ty,
            reasoning.super_path(),
            &fixture.builtin_types,
            &mut fixture.arena,
        );

        assert!(opt_sub_leaf.is_some());
        assert!(opt_super_leaf.is_some());
    }
}
