//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_ensure_free_variables_are_generialized_across_function_boundaries() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
-- Roughly taken from react-shallow-renderer
function createUpdater(renderer)
    local updater = {
        _renderer = renderer,
    }

    function updater.enqueueForceUpdate(publicInstance, callback, _callerName)
        updater._renderer.render(
            updater._renderer,
            updater._renderer._element,
            updater._renderer._context
        )
    end

    function updater.enqueueReplaceState(
        publicInstance,
        completeState,
        callback,
        _callerName
    )
        updater._renderer.render(
            updater._renderer,
            updater._renderer._element,
            updater._renderer._context
        )
    end

    function updater.enqueueSetState(publicInstance, partialState, callback, _callerName)
        local currentState = updater._renderer._newState or publicInstance.state
        updater._renderer.render(
            updater._renderer,
            updater._renderer._element,
            updater._renderer._context
        )
    end

    return updater
end

local ReactShallowRenderer = {}

function ReactShallowRenderer:_reset()
    self._updater = createUpdater(self)
end

return ReactShallowRenderer
    "#,
        ),
    );

    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local ReactShallowRenderer = require(game.A);
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
