//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_correct_stringification_user_defined_type_functions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use alloc::vec;
    use luaur_analysis::enums::reduction::Reduction;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_function::TypeFunction;
    use luaur_analysis::records::type_function_instance_type::TypeFunctionInstanceType;
    use luaur_analysis::records::type_function_reduction_result::TypeFunctionReductionResult;
    use luaur_ast::records::ast_name::AstName;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().numberType;
    let user = TypeFunction {
        name: String::from("user"),
        reducer: |_, _, _, _| TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        },
        can_reduce_generics: false,
    };
    let user_func_name = b"woohoo\0";
    let tftt = TypeFunctionInstanceType::new_user_defined(
        &user,
        vec![number_type],
        vec![],
        AstName::ast_name_c_char(user_func_name.as_ptr().cast()),
    );
    let tv = Type::from(tftt);

    assert_eq!("woohoo<number>", to_string_type_item(&tv));
}
