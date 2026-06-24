use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;

#[cfg(test)]
#[test]
fn subtyping_basic_irreducible_sub_type_function() {
    let mut fixture = SubtypeFixture::default();
    let boolean_ty = fixture.builtin_types.booleanType;
    let number_ty = fixture.builtin_types.numberType;
    let string_ty = fixture.builtin_types.stringType;

    let type_function_num_instance =
        TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
            &fixture.builtin_types.typeFunctions.add_func,
            vec![string_ty, boolean_ty],
        );
    let type_function_num = fixture.arena.add_type(type_function_num_instance);
    let result = fixture.is_subtype_type_id_type_id(type_function_num, number_ty);

    assert!(result.is_subtype());
}
