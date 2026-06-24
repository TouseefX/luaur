//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_signal_exerpt() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    fixture.get_frontend();

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Signal = {}
        Signal.ClassName = "Signal"
        export type Signal<T...> = typeof(setmetatable(
            {} :: {},
            {} :: typeof({ __index = Signal })
        ))
        function Signal.new<T...>(): Signal<T...>
            return nil :: any
        end

        function Signal.Connect<T...>(self: Signal<T...>)
        end

        function Signal.DisconnectAll<T...>(self: Signal<T...>): ()
            self._handlerListHead = false
        end

        function Signal.Fire<T...>(self: Signal<T...>): ()
            local connection
            rawget(connection, "_signal")
        end

        function Signal.Wait<T...>(self: Signal<T...>)
            connection = self:Connect(function()
                connection:Disconnect()
            end)
        end

        function Signal.Once<T...>(self: Signal<T...>, fn: SignalHandler<T...>): Connection<T...>
            connection = self:Connect(function() end)
        end
    "#,
        ),
        None,
    );

    let _ = result;
}
