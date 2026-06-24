use crate::records::subtype_fixture::SubtypeFixture;
use alloc::vec;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::property_type::Property;
use luaur_analysis::type_aliases::type_id::TypeId;

fn make_the_type(fixture: &mut SubtypeFixture) -> TypeId {
    let string_ty = fixture.builtin_types.stringType;
    let number_ty = fixture.builtin_types.numberType;
    let arg_type = fixture.tbl_with_indexer(
        SubtypeFixture::props(vec![("a", Property::rw_type_id(string_ty))]),
        string_ty,
        number_ty,
    );
    let arg_pack = fixture.pack_initializer_list_type_id(vec![arg_type]);
    let empty_type_pack = fixture.builtin_types.emptyTypePack;

    fixture.arena.add_type(FunctionType::function_type_new(
        arg_pack,
        empty_type_pack,
        None,
        false,
    ))
}

#[cfg(test)]
#[test]
fn subtyping_string_number_a_string_string_number_a_string() {
    let mut fixture = SubtypeFixture::default();

    let a = make_the_type(&mut fixture);
    let b = make_the_type(&mut fixture);

    assert!(fixture.is_subtype_type_id_type_id(a, b).is_subtype());
}
