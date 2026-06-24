use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;

#[cfg(test)]
#[test]
fn subtyping_basic_type_function_with_generics() {
    let mut fixture = SubtypeFixture::default();
    let generic_t = fixture.generic("T");
    let generic_u = fixture.generic("U");
    let number_ty = fixture.builtin_types.numberType;

    let add_type_function_instance =
        TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
            &fixture.builtin_types.typeFunctions.add_func,
            vec![generic_t, generic_u],
        );
    let add_type_function = fixture.arena.add_type(add_type_function_instance);
    let function_arg_pack = fixture.pack_initializer_list_type_id(vec![generic_t, generic_u]);
    let function_ret_pack = fixture.pack_initializer_list_type_id(vec![add_type_function]);
    let function_type = fixture.arena.add_type(FunctionType::new_with_generics(
        vec![generic_t, generic_u],
        vec![],
        function_arg_pack,
        function_ret_pack,
        None,
        false,
    ));
    let super_function = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty, number_ty],
        vec![number_ty],
    );
    let result = fixture.is_subtype_type_id_type_id(function_type, super_function);

    assert!(result.is_subtype());
}
