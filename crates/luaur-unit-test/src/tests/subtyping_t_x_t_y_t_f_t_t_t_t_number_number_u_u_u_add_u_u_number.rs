use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;

#[cfg(test)]
#[test]
fn subtyping_t_x_t_y_t_f_t_t_t_t_number_number_u_u_u_add_u_u_number() {
    let mut fixture = SubtypeFixture::default();
    let generic_t = fixture.generic("T");
    let generic_u = fixture.generic("U");
    let number_ty = fixture.builtin_types.numberType;

    let generic_t_binary_fn = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![generic_t, generic_t],
        vec![generic_t],
    );
    let f1_arg_pack =
        fixture.pack_initializer_list_type_id(vec![generic_t, generic_t, generic_t_binary_fn]);
    let f1_ret_pack = fixture.pack_initializer_list_type_id(vec![generic_t]);
    let f1 = fixture.arena.add_type(FunctionType::new_with_generics(
        vec![generic_t],
        vec![],
        f1_arg_pack,
        f1_ret_pack,
        None,
        false,
    ));

    let add_u_to_u_instance =
        TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
            &fixture.builtin_types.typeFunctions.add_func,
            vec![generic_u, generic_u],
        );
    let add_u_to_u = fixture.arena.add_type(add_u_to_u_instance);
    let generic_u_arg_pack = fixture.pack_initializer_list_type_id(vec![generic_u, generic_u]);
    let generic_u_ret_pack = fixture.pack_initializer_list_type_id(vec![add_u_to_u]);
    let generic_u_binary_add = fixture.arena.add_type(FunctionType::new_with_generics(
        vec![generic_u],
        vec![],
        generic_u_arg_pack,
        generic_u_ret_pack,
        None,
        false,
    ));
    let f2 = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![number_ty, number_ty, generic_u_binary_add],
        vec![number_ty],
    );

    assert!(fixture.is_subtype_type_id_type_id(f1, f2).is_subtype());
}
