//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:470:type_infer_oop_promise_type_error_too_complex`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_oop_promise_type_error_too_complex

#[cfg(test)]
#[test]
fn type_infer_oop_promise_type_error_too_complex() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        local Promise = {}
        Promise.prototype = {}
        Promise.__index = Promise.prototype

        function Promise._new(traceback, callback, parent)
            if parent ~= nil and not Promise.is(parent)then
            end

            local self = {
                _parent = parent,
            }

            parent._consumers[self] = true
            setmetatable(self, Promise)
            self:_reject()

            return self
        end

        function Promise.resolve(...)
            return Promise._new(debug.traceback(nil, 2), function(resolve)
            end)
        end

        function Promise.reject(...)
            return Promise._new(debug.traceback(nil, 2), function(_, reject)
            end)
        end

        function Promise._try(traceback, callback, ...)
            return Promise._new(traceback, function(resolve)
            end)
        end

        function Promise.try(callback, ...)
            return Promise._try(debug.traceback(nil, 2), callback, ...)
        end

        function Promise._all(traceback, promises, amount)
            if #promises == 0 or amount == 0 then
                return Promise.resolve({})
            end
            return Promise._new(traceback, function(resolve, reject, onCancel)
            end)
        end

        function Promise.all(promises)
            return Promise._all(debug.traceback(nil, 2), promises)
        end

        function Promise.allSettled(promises)
            return Promise.resolve({})
        end

        function Promise.race(promises)
            return Promise._new(debug.traceback(nil, 2), function(resolve, reject, onCancel)
            end)
        end

        function Promise.each(list, predicate)
            return Promise._new(debug.traceback(nil, 2), function(resolve, reject, onCancel)
                local predicatePromise = Promise.resolve(predicate(value, index))
                local success, result = predicatePromise:await()
            end)
        end

        function Promise.is(object)
        end

        function Promise.prototype:_reject(...)
            self:_finalize()
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
