use crate::records::subtype_fixture::SubtypeFixture;
use alloc::{string::String, sync::Arc, vec};
use luaur_analysis::enums::polarity::Polarity;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::generic_type::GenericType;
use luaur_analysis::records::scope::Scope;

#[cfg(test)]
#[test]
fn subtyping_substitute_a_generic_for_a_negation() {
    let mut fixture = SubtypeFixture::default();
    let scope = Arc::as_ptr(&fixture.module_scope) as *mut Scope;
    let truthy_ty = fixture.builtin_types.truthyType;

    let a_ty = fixture
        .arena
        .add_type(GenericType::generic_type_scope_name_polarity(
            scope,
            String::from("A"),
            Polarity::Mixed,
        ));
    let b_ty = fixture
        .arena
        .add_type(GenericType::generic_type_scope_name_polarity(
            scope,
            String::from("B"),
            Polarity::Mixed,
        ));

    let generic_ret_left = fixture.meet(a_ty, truthy_ty);
    let generic_ret = fixture.join(generic_ret_left, b_ty);
    let generic_arg_pack = fixture.pack_initializer_list_type_id(vec![a_ty, b_ty]);
    let generic_ret_pack = fixture.pack_initializer_list_type_id(vec![generic_ret]);
    let generic_function_ty = fixture.arena.add_type(FunctionType::new_with_generics(
        vec![a_ty, b_ty],
        vec![],
        generic_arg_pack,
        generic_ret_pack,
        None,
        false,
    ));

    let actual_ret_left = fixture.meet(truthy_ty, truthy_ty);
    let actual_ret = fixture.join(actual_ret_left, truthy_ty);
    let actual_function_ty = fixture.fn_item_initializer_list_type_id_initializer_list_type_id(
        vec![truthy_ty, truthy_ty],
        vec![actual_ret],
    );

    let result = fixture.is_subtype_type_id_type_id(generic_function_ty, actual_function_ty);

    assert!(result.is_subtype());
}
