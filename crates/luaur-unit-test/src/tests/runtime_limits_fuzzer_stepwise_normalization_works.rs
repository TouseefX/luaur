//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_fuzzer_stepwise_normalization_works() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        _ = if _ then {n0=# _,[_]=_,``,[function(l0,l0,l0)
        do end
        end]=_,setmetatable,[l0(_ + _)]=_,} else _(),_,_
        _[_](_,_(coroutine,_,_,nil),_(0,_()),function()
        end)
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
