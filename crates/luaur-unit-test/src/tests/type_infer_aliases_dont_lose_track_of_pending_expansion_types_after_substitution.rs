//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_dont_lose_track_of_pending_expansion_types_after_substitution() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.base.file_resolver.source.insert(
        String::from("game/ReactCurrentDispatcher"),
        String::from(
            r#"
        export type BasicStateAction<S> = ((S) -> S) | S
        export type Dispatch<A> = (A) -> ()

        export type Dispatcher = {
            useState: <S>(initialState: (() -> S) | S) -> (S, Dispatch<BasicStateAction<S>>),
        }

        return {}
    "#,
        ),
    );
    fixture.base.file_resolver.source.insert(
        String::from("game/React/React/ReactHooks"),
        String::from(
            r#"
        local RCD = require(script.Parent.Parent.Parent.ReactCurrentDispatcher)

        local function resolveDispatcher(): RCD.Dispatcher
            return (nil :: any) :: RCD.Dispatcher
        end

        function useState<S>(
            initialState: (() -> S) | S
        ): (S, RCD.Dispatch<RCD.BasicStateAction<S>>)
            local dispatcher = resolveDispatcher()
            return dispatcher.useState(initialState)
        end
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(
            &String::from("game/React/React/ReactHooks"),
            None,
        );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
