//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_require_a_variadic_function() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::begin_type_pack;
    use luaur_analysis::functions::end_type_pack;
    use luaur_analysis::functions::follow_type::follow;
    use luaur_analysis::functions::get_type_id::get_type_id;
    use luaur_analysis::functions::get_type_pack::get_type_pack_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::variadic_type_pack::VariadicTypePack;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
        local T = {}
        function T.f(...) end
        return T
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("game/B"),
        String::from(
            r#"
        local A = require(game.A)
        local f = A.f
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/B"), None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let b_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/B"));
    let f = unsafe {
        follow(
            fixture
                .base
                .require_type_module_ptr_string(&b_module, &String::from("f")),
        )
    };

    let ftv = unsafe { get_type_id::<FunctionType>(f).as_ref() }.expect("expected function type");
    let iter = begin_type_pack::begin(ftv.arg_types());
    let end_iter = end_type_pack::end(ftv.arg_types());

    assert!(iter.operator_eq(&end_iter));
    let tail = iter.tail().expect("expected variadic argument tail");
    assert!(!unsafe { get_type_pack_id::<VariadicTypePack>(tail) }.is_null());
}
