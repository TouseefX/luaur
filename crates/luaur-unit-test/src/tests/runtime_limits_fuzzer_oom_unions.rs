//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_fuzzer_oom_unions() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local _ = true,l0
        _ = if _ then _ else _._,if _[_] then nil elseif _ then `` else _._,...
        _ = if _ then _ elseif _ then `` else _.n0,true,...
        _G = if "" then _ else _.n0,_
        _ = if _[_] then _ elseif _ then _ + n0 else _._,32804,...
        _.readstring = _,_
        local l0 = require(module0)
        _ = _,l0,_
        do end
        _.readstring += _
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
