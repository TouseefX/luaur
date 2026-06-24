use crate::records::subtype_fixture::SubtypeFixture;
use alloc::{string::String, sync::Arc};
use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
use luaur_analysis::records::free_type::FreeType;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::singleton_type::SingletonType;
use luaur_analysis::records::string_singleton::StringSingleton;
use luaur_analysis::type_aliases::singleton_variant::SingletonVariant;

#[cfg(test)]
#[test]
fn subtyping_free_types_might_be_subtypes() {
    let mut fixture = SubtypeFixture::default();
    let scope = Arc::as_ptr(&fixture.module_scope) as *mut Scope;
    let string_ty = fixture.builtin_types.stringType;

    let arg_ty = fixture
        .arena
        .fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, scope);
    let five_ty = fixture
        .arena
        .add_type(SingletonType::singleton_type(SingletonVariant::V1(
            StringSingleton::new(String::from("five")),
        )));
    let free_arg = unsafe { get_mutable_type_id::<FreeType>(arg_ty).as_mut() };
    let free_arg = free_arg.expect("fresh type should be a FreeType");
    free_arg.lower_bound = five_ty;
    free_arg.upper_bound = string_ty;

    let result = fixture.is_subtype_type_id_type_id(string_ty, arg_ty);

    assert!(result.is_subtype());
    assert_eq!(1, result.assumed_constraints().len());
}
