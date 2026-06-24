//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_fusion_normalization_spin() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
type Task = unknown
type Constructors = unknown
export type Scope<Constructors = any> = {Task} & Constructors
export type DeriveScopeConstructor = (<S>(Scope<S>) -> Scope<S>)
    & (<S, A>(Scope<S>, A & {}) -> Scope<S & A>)
    & (<S, A, B>(Scope<S>, A & {}, B & {}) -> Scope<S & A & B>)
    & (<S, A, B, C>(Scope<S>, A & {}, B & {}, C & {}) -> Scope<S & A & B & C>)
    & (<S, A, B, C, D>(Scope<S>, A & {}, B & {}, C & {}, D & {}) -> Scope<S & A & B & C & D>)
    & (<S, A, B, C, D, E>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}) -> Scope<S & A & B & C & D & E>)
    & (<S, A, B, C, D, E, F>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}) -> Scope<S & A & B & C & D & E & F>)
    & (<S, A, B, C, D, E, F, G>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}) -> Scope<S & A & B & C & D & E & F & G>)
    & (<S, A, B, C, D, E, F, G, H>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}) -> Scope<S & A & B & C & D & E & F & G & H>)
    & (<S, A, B, C, D, E, F, G, H, I>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}, I & {}) -> Scope<S & A & B & C & D & E & F & G & H & I>)
    & (<S, A, B, C, D, E, F, G, H, I, J>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}, I & {}, J & {}) -> Scope<S & A & B & C & D & E & F & G & H & I & J>)
    & (<S, A, B, C, D, E, F, G, H, I, J, K>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}, I & {}, J & {}, K & {}) -> Scope<S & A & B & C & D & E & F & G & H & I & J & K>)
    & (<S, A, B, C, D, E, F, G, H, I, J, K, L>(Scope<S>, A & {}, B & {}, C & {}, D & {}, E & {}, F & {}, G & {}, H & {}, I & {}, J & {}, K & {}, L & {}) -> Scope<S & A & B & C & D & E & F & G & H & I & J & K & L>)

local deriveScopeImpl : DeriveScopeConstructor = (nil :: any)

local function innerScope<T>(
    existing: Types.Scope<T>,
    ...: {[unknown]: unknown}
): any
    local new = deriveScopeImpl(existing, ...)
end

    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
