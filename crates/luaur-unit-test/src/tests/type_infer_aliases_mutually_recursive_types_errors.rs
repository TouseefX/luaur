//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_mutually_recursive_types_errors() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::copy_errors::copy_errors;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::unfreeze::unfreeze;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        type T<a> = { f: a, g: U<a> }
        type U<b> = { h: b, i: T<b>? }
        local x: T<number> = { f = 37, g = { h = 5, i = nil } }
        x.g.i = x
        local y: T<string> = { f = "hi", g = { h = 5, i = nil } }
        y.g.i = y
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let module = unsafe { &mut *fixture.get_main_module(false) };
    unfreeze(&mut module.interface_types);
    copy_errors(&mut module.errors, &mut module.interface_types, unsafe {
        &*fixture.builtin_types
    });
    freeze(&mut module.interface_types);
    module.internal_types.clear();
    module.ast_types.clear();

    for error in &module.errors {
        let error_string = to_string_type_error(error);
        assert!(
            !error_string.contains("VALUELESS"),
            "unexpected VALUELESS in {error_string}"
        );
    }
}
