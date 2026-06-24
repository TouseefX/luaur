use crate::records::subtype_fixture::SubtypeFixture;
use alloc::{string::String, vec};
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::records::singleton_type::SingletonType;
use luaur_analysis::records::string_singleton::StringSingleton;
use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;
use luaur_analysis::type_aliases::singleton_variant::SingletonVariant;

#[cfg(test)]
#[test]
fn subtyping_no_caching_type_function_instances_with_mapped_generics() {
    let mut fixture = SubtypeFixture::default();
    let generic_u = fixture.generic("U");
    let number_ty = fixture.builtin_types.numberType;

    let key_of_u_instance =
        TypeFunctionInstanceType::type_function_instance_type_type_function_vector_type_id(
            &fixture.builtin_types.typeFunctions.keyof_func,
            vec![generic_u],
        );
    let key_of_u = fixture.arena.add_type(key_of_u_instance);
    let u_arg_pack = fixture.pack_initializer_list_type_id(vec![generic_u]);
    let u_ret_pack = fixture.pack_initializer_list_type_id(vec![key_of_u]);
    let u_to_key_of_u = fixture.arena.add_type(FunctionType::new_with_generics(
        vec![generic_u],
        vec![],
        u_arg_pack,
        u_ret_pack,
        None,
        false,
    ));
    let sub_type_pack = fixture.pack_initializer_list_type_id(vec![u_to_key_of_u, u_to_key_of_u]);

    let tbl_a = fixture.tbl(SubtypeFixture::props(vec![(
        "a",
        Property::rw_type_id(number_ty),
    )]));
    let tbl_b = fixture.tbl(SubtypeFixture::props(vec![(
        "b",
        Property::rw_type_id(number_ty),
    )]));
    let a_singleton = fixture
        .arena
        .add_type(SingletonType::singleton_type(SingletonVariant::V1(
            StringSingleton::new(String::from("a")),
        )));
    let tbl_a_to_a_singleton = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(vec![tbl_a], vec![a_singleton]);
    let tbl_b_to_a_singleton = fixture
        .fn_item_initializer_list_type_id_initializer_list_type_id(vec![tbl_b], vec![a_singleton]);
    let super_type_pack =
        fixture.pack_initializer_list_type_id(vec![tbl_a_to_a_singleton, tbl_b_to_a_singleton]);

    assert!(!fixture
        .is_subtype_type_pack_id_type_pack_id(sub_type_pack, super_type_pack)
        .is_subtype());
}
