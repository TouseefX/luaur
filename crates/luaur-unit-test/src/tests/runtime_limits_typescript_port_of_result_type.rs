//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_typescript_port_of_result_type() {
    use crate::functions::has_error::has_error;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::records::code_too_complex::CodeTooComplex;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let source = String::from(
        r#"
        --!strict

        -- Big thanks to Dionysusnu by letting us use this code as part of our test suite!
        -- https://github.com/Dionysusnu/rbxts-rust-classes
        -- Licensed under the MPL 2.0: https://raw.githubusercontent.com/Dionysusnu/rbxts-rust-classes/master/LICENSE

        local TS = _G[script]
        local lazyGet = TS.import(script, script.Parent.Parent, "util", "lazyLoad").lazyGet
        local unit = TS.import(script, script.Parent.Parent, "util", "Unit").unit
        local Iterator
        lazyGet("Iterator", function(c)
            Iterator = c
        end)
        local Option
        lazyGet("Option", function(c)
            Option = c
        end)
        local Vec
        lazyGet("Vec", function(c)
            Vec = c
        end)
        local Result
        do
            Result = setmetatable({}, {
                __tostring = function()
                    return "Result"
                end,
            })
            Result.__index = Result
            function Result.new(...)
                local self = setmetatable({}, Result)
                self:constructor(...)
                return self
            end
            function Result:constructor(okValue, errValue)
                self.okValue = okValue
                self.errValue = errValue
            end
            function Result:ok(val)
                return Result.new(val, nil)
            end
            function Result:err(val)
                return Result.new(nil, val)
            end
            function Result:fromCallback(c)
                local _0 = c
                local _1, _2 = pcall(_0)
                local result = _1 and {
                    success = true,
                    value = _2,
                } or {
                    success = false,
                    error = _2,
                }
                return result.success and Result:ok(result.value) or Result:err(Option:wrap(result.error))
            end
            function Result:fromVoidCallback(c)
                local _0 = c
                local _1, _2 = pcall(_0)
                local result = _1 and {
                    success = true,
                    value = _2,
                } or {
                    success = false,
                    error = _2,
                }
                return result.success and Result:ok(unit()) or Result:err(Option:wrap(result.error))
            end
            Result.fromPromise = TS.async(function(self, p)
                local _0, _1 = TS.try(function()
                    return TS.TRY_RETURN, { Result:ok(TS.await(p)) }
                end, function(e)
                    return TS.TRY_RETURN, { Result:err(Option:wrap(e)) }
                end)
                if _0 then
                    return unpack(_1)
                end
            end)
            Result.fromVoidPromise = TS.async(function(self, p)
                local _0, _1 = TS.try(function()
                    TS.await(p)
                    return TS.TRY_RETURN, { Result:ok(unit()) }
                end, function(e)
                    return TS.TRY_RETURN, { Result:err(Option:wrap(e)) }
                end)
                if _0 then
                    return unpack(_1)
                end
            end)
            function Result:isOk()
                return self.okValue ~= nil
            end
            function Result:isErr()
                return self.errValue ~= nil
            end
            function Result:contains(x)
                return self.okValue == x
            end
            function Result:containsErr(x)
                return self.errValue == x
            end
            function Result:okOption()
                return Option:wrap(self.okValue)
            end
            function Result:errOption()
                return Option:wrap(self.errValue)
            end
            function Result:map(func)
                return self:isOk() and Result:ok(func(self.okValue)) or Result:err(self.errValue)
            end
            function Result:mapOr(def, func)
                local _0
                if self:isOk() then
                    _0 = func(self.okValue)
                else
                    _0 = def
                end
                return _0
            end
            function Result:mapOrElse(def, func)
                local _0
                if self:isOk() then
                    _0 = func(self.okValue)
                else
                    _0 = def(self.errValue)
                end
                return _0
            end
            function Result:mapErr(func)
                return self:isErr() and Result:err(func(self.errValue)) or Result:ok(self.okValue)
            end
            Result["and"] = function(self, other)
                return self:isErr() and Result:err(self.errValue) or other
            end
            function Result:andThen(func)
                return self:isErr() and Result:err(self.errValue) or func(self.okValue)
            end
            Result["or"] = function(self, other)
                return self:isOk() and Result:ok(self.okValue) or other
            end
            function Result:orElse(other)
                return self:isOk() and Result:ok(self.okValue) or other(self.errValue)
            end
            function Result:expect(msg)
                if self:isOk() then
                    return self.okValue
                else
                    error(msg)
                end
            end
            function Result:unwrap()
                return self:expect("called `Result.unwrap()` on an `Err` value: " .. tostring(self.errValue))
            end
            function Result:unwrapOr(def)
                local _0
                if self:isOk() then
                    _0 = self.okValue
                else
                    _0 = def
                end
                return _0
            end
            function Result:unwrapOrElse(gen)
                local _0
                if self:isOk() then
                    _0 = self.okValue
                else
                    _0 = gen(self.errValue)
                end
                return _0
            end
            function Result:expectErr(msg)
                if self:isErr() then
                    return self.errValue
                else
                    error(msg)
                end
            end
            function Result:unwrapErr()
                return self:expectErr("called `Result.unwrapErr()` on an `Ok` value: " .. tostring(self.okValue))
            end
            function Result:transpose()
                return self:isOk() and self.okValue:map(function(some)
                    return Result:ok(some)
                end) or Option:some(Result:err(self.errValue))
            end
            function Result:flatten()
                return self:isOk() and Result.new(self.okValue.okValue, self.okValue.errValue) or Result:err(self.errValue)
            end
            function Result:match(ifOk, ifErr)
                local _0
                if self:isOk() then
                    _0 = ifOk(self.okValue)
                else
                    _0 = ifErr(self.errValue)
                end
                return _0
            end
            function Result:asPtr()
                local _0 = (self.okValue)
                if _0 == nil then
                    _0 = (self.errValue)
                end
                return _0
            end
        end
        local resultMeta = Result
        resultMeta.__eq = function(a, b)
            return b:match(function(ok)
                return a:contains(ok)
            end, function(err)
                return a:containsErr(err)
            end)
        end
        resultMeta.__tostring = function(result)
            return result:match(function(ok)
                return "Result.ok(" .. tostring(ok) .. ")"
            end, function(err)
                return "Result.err(" .. tostring(err) .. ")"
            end)
        end
        return {
            Result = Result,
        }
    "#,
    );

    let result = fixture
        .base
        .check_string_optional_frontend_options(&source, None);

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    if FFlag::DebugLuauForceOldSolver.get() {
        assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
    }
}
