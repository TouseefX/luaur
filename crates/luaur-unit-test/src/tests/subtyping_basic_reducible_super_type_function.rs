use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;

#[cfg(test)]
#[test]
fn subtyping_basic_reducible_super_type_function() {
    let mut fixture = SubtypeFixture::default();
    let number_ty = fixture.builtin_types.numberType;

    let type_function_num_instance =
        TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
            &fixture.builtin_types.typeFunctions.add_func,
            vec![number_ty, number_ty],
        );
    let type_function_num = fixture.arena.add_type(type_function_num_instance);
    let result = fixture.is_subtype_type_id_type_id(number_ty, type_function_num);

    assert!(result.is_subtype());
}
