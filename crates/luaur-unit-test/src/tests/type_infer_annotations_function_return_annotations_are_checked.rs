//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_function_return_annotations_are_checked() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::follow_type_pack::follow_type_pack_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::get_type_pack::get_type_pack_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::type_pack::TypePack;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function fifty(): any
            return 55
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let fifty_type = fixture.require_type_string(&String::from("fifty"));
    let ftv = unsafe { get_type_id::<FunctionType>(fifty_type).as_ref() }
        .expect("expected function type");

    let ret_pack = unsafe { follow_type_pack_id(ftv.ret_types()) };
    let tp = unsafe { get_type_pack_id::<TypePack>(ret_pack).as_ref() }
        .expect("expected return type pack");

    assert_eq!(1, tp.head().len());
    assert_eq!(unsafe { (*fixture.get_builtins()).anyType }, unsafe {
        follow_type_id(tp.head()[0])
    });
}
